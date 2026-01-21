use crate::executor::TestExecutor;
use crate::models::{TestCase, TestRun, TestRunStatus};
use crate::storage::TestCaseStorage;
use crate::test_run_storage::TestRunStorage;
use crate::verification::{TestCaseVerificationResult, TestExecutionLog, TestVerifier};
use anyhow::{Context, Result};
use chrono::Utc;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryStrategy {
    NoRetry,
    FixedRetries { max_attempts: usize },
    ExponentialBackoff { max_attempts: usize, base_delay_ms: u64 },
}

#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub strategy: RetryStrategy,
    pub retry_on_failure_only: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            strategy: RetryStrategy::NoRetry,
            retry_on_failure_only: true,
        }
    }
}

impl RetryPolicy {
    pub fn no_retry() -> Self {
        Self::default()
    }

    pub fn fixed_retries(max_attempts: usize) -> Self {
        Self {
            strategy: RetryStrategy::FixedRetries { max_attempts },
            retry_on_failure_only: true,
        }
    }

    pub fn exponential_backoff(max_attempts: usize, base_delay_ms: u64) -> Self {
        Self {
            strategy: RetryStrategy::ExponentialBackoff {
                max_attempts,
                base_delay_ms,
            },
            retry_on_failure_only: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkerConfig {
    pub num_workers: usize,
    pub retry_policy: RetryPolicy,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            num_workers: 4,
            retry_policy: RetryPolicy::default(),
        }
    }
}

impl WorkerConfig {
    pub fn new(num_workers: usize) -> Self {
        Self {
            num_workers,
            retry_policy: RetryPolicy::default(),
        }
    }

    pub fn with_retry_policy(mut self, retry_policy: RetryPolicy) -> Self {
        self.retry_policy = retry_policy;
        self
    }
}

#[derive(Debug, Clone)]
pub struct TestExecutionResult {
    pub test_case_id: String,
    pub success: bool,
    pub duration_s: f64,
    pub attempts: usize,
    pub error_message: Option<String>,
    pub execution_log: String,
}

#[derive(Debug, Clone)]
pub struct OrchestratorStats {
    pub total_tests: usize,
    pub completed_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub running_tests: usize,
    pub total_attempts: usize,
    pub elapsed_time_ms: u64,
}

impl OrchestratorStats {
    pub fn new(total_tests: usize) -> Self {
        Self {
            total_tests,
            completed_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
            running_tests: 0,
            total_attempts: 0,
            elapsed_time_ms: 0,
        }
    }

    pub fn success_rate(&self) -> f64 {
        if self.completed_tests == 0 {
            0.0
        } else {
            (self.passed_tests as f64 / self.completed_tests as f64) * 100.0
        }
    }

    pub fn progress_percentage(&self) -> f64 {
        if self.total_tests == 0 {
            100.0
        } else {
            (self.completed_tests as f64 / self.total_tests as f64) * 100.0
        }
    }
}

pub struct ProgressReporter {
    stats: Arc<Mutex<OrchestratorStats>>,
    start_time: Instant,
    stop_signal: Arc<AtomicBool>,
    verbose: bool,
}

impl ProgressReporter {
    pub fn new(total_tests: usize, verbose: bool) -> Self {
        Self {
            stats: Arc::new(Mutex::new(OrchestratorStats::new(total_tests))),
            start_time: Instant::now(),
            stop_signal: Arc::new(AtomicBool::new(false)),
            verbose,
        }
    }

    pub fn get_stats(&self) -> Arc<Mutex<OrchestratorStats>> {
        Arc::clone(&self.stats)
    }

    pub fn start_live_display(&self) {
        let stats = Arc::clone(&self.stats);
        let stop_signal = Arc::clone(&self.stop_signal);
        let start_time = self.start_time;
        let verbose = self.verbose;

        thread::spawn(move || {
            while !stop_signal.load(Ordering::Relaxed) {
                Self::print_progress(&stats, start_time, verbose);
                thread::sleep(Duration::from_millis(500));
            }
            Self::print_progress(&stats, start_time, verbose);
        });
    }

    pub fn stop(&self) {
        self.stop_signal.store(true, Ordering::Relaxed);
        thread::sleep(Duration::from_millis(600));
    }

    fn print_progress(stats: &Arc<Mutex<OrchestratorStats>>, start_time: Instant, verbose: bool) {
        let stats = stats.lock().unwrap();
        let elapsed = start_time.elapsed();

        print!("\r");
        if verbose {
            print!(
                "[{:02}:{:02}] Progress: {}/{} ({:.1}%) | Running: {} | Passed: {} | Failed: {} | Success: {:.1}% | Attempts: {}",
                elapsed.as_secs() / 60,
                elapsed.as_secs() % 60,
                stats.completed_tests,
                stats.total_tests,
                stats.progress_percentage(),
                stats.running_tests,
                stats.passed_tests,
                stats.failed_tests,
                stats.success_rate(),
                stats.total_attempts,
            );
        } else {
            print!(
                "[{:02}:{:02}] {}/{} ({:.1}%) | ✓ {} | ✗ {} | {:.1}%",
                elapsed.as_secs() / 60,
                elapsed.as_secs() % 60,
                stats.completed_tests,
                stats.total_tests,
                stats.progress_percentage(),
                stats.passed_tests,
                stats.failed_tests,
                stats.success_rate(),
            );
        }
        std::io::stdout().flush().unwrap();
    }
}

pub struct TestOrchestrator {
    test_case_storage: TestCaseStorage,
    test_run_storage: TestRunStorage,
    output_dir: PathBuf,
}

impl TestOrchestrator {
    pub fn new(
        test_case_storage: TestCaseStorage,
        test_run_storage: TestRunStorage,
        output_dir: PathBuf,
    ) -> Result<Self> {
        if !output_dir.exists() {
            fs::create_dir_all(&output_dir).context(format!(
                "Failed to create output directory: {}",
                output_dir.display()
            ))?;
        }

        Ok(Self {
            test_case_storage,
            test_run_storage,
            output_dir,
        })
    }

    pub fn select_test_cases(&self, test_case_ids: Vec<String>) -> Result<Vec<TestCase>> {
        let mut test_cases = Vec::new();

        for id in test_case_ids {
            let test_case = self
                .test_case_storage
                .load_test_case_by_id(&id)
                .context(format!("Failed to load test case: {}", id))?;
            test_cases.push(test_case);
        }

        Ok(test_cases)
    }

    pub fn select_all_test_cases(&self) -> Result<Vec<TestCase>> {
        self.test_case_storage
            .load_all_test_cases()
            .context("Failed to load all test cases")
    }

    pub fn execute_tests(
        &self,
        test_cases: Vec<TestCase>,
        config: WorkerConfig,
        verbose: bool,
    ) -> Result<Vec<TestExecutionResult>> {
        let total_tests = test_cases.len();
        let reporter = ProgressReporter::new(total_tests, verbose);
        let stats = reporter.get_stats();

        if total_tests == 0 {
            println!("No test cases to execute.");
            return Ok(Vec::new());
        }

        println!("\n=== Test Execution Starting ===");
        println!("Total test cases: {}", total_tests);
        println!("Worker threads: {}", config.num_workers);
        println!(
            "Retry policy: {}",
            match config.retry_policy.strategy {
                RetryStrategy::NoRetry => "No retries".to_string(),
                RetryStrategy::FixedRetries { max_attempts } =>
                    format!("Fixed retries (max {})", max_attempts),
                RetryStrategy::ExponentialBackoff {
                    max_attempts,
                    base_delay_ms,
                } => format!(
                    "Exponential backoff (max {}, base delay {}ms)",
                    max_attempts, base_delay_ms
                ),
            }
        );
        println!();

        reporter.start_live_display();

        let test_queue = Arc::new(Mutex::new(test_cases.into_iter().enumerate().collect::<Vec<_>>()));
        let results = Arc::new(Mutex::new(Vec::new()));
        let active_workers = Arc::new(AtomicUsize::new(0));

        let mut handles = vec![];

        for worker_id in 0..config.num_workers {
            let queue = Arc::clone(&test_queue);
            let results_ref = Arc::clone(&results);
            let stats_ref = Arc::clone(&stats);
            let active_workers_ref = Arc::clone(&active_workers);
            let retry_policy = config.retry_policy.clone();
            let output_dir = self.output_dir.clone();

            let handle = thread::spawn(move || {
                let executor = TestExecutor::new();

                loop {
                    let test_case = {
                        let mut queue = queue.lock().unwrap();
                        queue.pop()
                    };

                    match test_case {
                        Some((_index, test_case)) => {
                            active_workers_ref.fetch_add(1, Ordering::Relaxed);
                            {
                                let mut stats = stats_ref.lock().unwrap();
                                stats.running_tests += 1;
                            }

                            let result = Self::execute_test_with_retry(
                                &executor,
                                &test_case,
                                &retry_policy,
                                &stats_ref,
                                worker_id,
                                &output_dir,
                            );

                            {
                                let mut stats = stats_ref.lock().unwrap();
                                stats.running_tests -= 1;
                                stats.completed_tests += 1;
                                if result.success {
                                    stats.passed_tests += 1;
                                } else {
                                    stats.failed_tests += 1;
                                }
                            }

                            {
                                let mut results = results_ref.lock().unwrap();
                                results.push(result);
                            }

                            active_workers_ref.fetch_sub(1, Ordering::Relaxed);
                        }
                        None => break,
                    }
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Worker thread panicked");
        }

        reporter.stop();
        println!("\n");

        let final_results = {
            let results = results.lock().unwrap();
            results.clone()
        };

        let final_stats = stats.lock().unwrap();
        self.print_summary(&final_stats);

        Ok(final_results)
    }

    fn execute_test_with_retry(
        executor: &TestExecutor,
        test_case: &TestCase,
        retry_policy: &RetryPolicy,
        stats: &Arc<Mutex<OrchestratorStats>>,
        _worker_id: usize,
        output_dir: &Path,
    ) -> TestExecutionResult {
        let max_attempts = match retry_policy.strategy {
            RetryStrategy::NoRetry => 1,
            RetryStrategy::FixedRetries { max_attempts } => max_attempts,
            RetryStrategy::ExponentialBackoff { max_attempts, .. } => max_attempts,
        };

        let mut last_result = None;
        let mut total_duration_s = 0.0f64;

        for attempt in 1..=max_attempts {
            {
                let mut stats = stats.lock().unwrap();
                stats.total_attempts += 1;
            }

            let start = Instant::now();
            let result = executor.execute_test_case(test_case);
            let duration = start.elapsed();
            let duration_s = duration.as_secs_f64();
            total_duration_s += duration_s;

            let success = result.is_ok();

            let (execution_log, error_message) = match result {
                Ok(_) => (format!("Test case {} executed successfully", test_case.id), None),
                Err(e) => (
                    format!("Test case {} failed", test_case.id),
                    Some(e.to_string()),
                ),
            };

            let log_path = output_dir.join(format!(
                "{}_attempt{}.log",
                test_case.id.replace('/', "_"),
                attempt
            ));
            let _ = fs::write(
                &log_path,
                format!(
                    "Test Case: {}\nAttempt: {}/{}\nSuccess: {}\nDuration: {}ms\nLog:\n{}\nError:\n{}\n",
                    test_case.id,
                    attempt,
                    max_attempts,
                    success,
                    duration_s,
                    execution_log,
                    error_message.as_deref().unwrap_or("None")
                ),
            );

            last_result = Some(TestExecutionResult {
                test_case_id: test_case.id.clone(),
                success,
                duration_s,
                attempts: attempt,
                error_message,
                execution_log,
            });

            if success {
                break;
            }

            if retry_policy.retry_on_failure_only && attempt < max_attempts {
                if let RetryStrategy::ExponentialBackoff { base_delay_ms, .. } =
                    retry_policy.strategy
                {
                    let delay_ms = base_delay_ms * 2_u64.pow((attempt - 1) as u32);
                    thread::sleep(Duration::from_millis(delay_ms));
                }
            }
        }

        last_result.unwrap()
    }

    pub fn save_results(&self, results: &[TestExecutionResult]) -> Result<()> {
        for result in results {
            let test_run = TestRun {
                name: None,
                test_case_id: result.test_case_id.clone(),
                timestamp: Utc::now(),
                status: if result.success {
                    TestRunStatus::Pass
                } else {
                    TestRunStatus::Fail
                },
                duration: result.duration_s,
                execution_log: result.execution_log.clone(),
                error_message: result.error_message.clone(),
            };

            self.test_run_storage
                .save_test_run(&test_run)
                .context(format!(
                    "Failed to save test run for {}",
                    result.test_case_id
                ))?;
        }

        Ok(())
    }

    pub fn verify_results(
        &self,
        log_files: Vec<PathBuf>,
    ) -> Result<Vec<TestCaseVerificationResult>> {
        let verifier = TestVerifier::new(self.test_case_storage.clone());
        let mut verification_results = Vec::new();

        for log_file in log_files {
            let logs = verifier
                .parse_log_file(&log_file)
                .context(format!("Failed to parse log file: {}", log_file.display()))?;

            let test_case_logs: HashMap<String, Vec<TestExecutionLog>> = {
                let mut map: HashMap<String, Vec<TestExecutionLog>> = HashMap::new();
                for log in logs {
                    map.entry(log.test_case_id.clone()).or_default().push(log);
                }
                map
            };

            for (test_case_id, logs) in test_case_logs {
                match self.test_case_storage.load_test_case_by_id(&test_case_id) {
                    Ok(test_case) => {
                        let result = verifier.verify_test_case(&test_case, &logs);
                        verification_results.push(result);
                    }
                    Err(e) => {
                        log::warn!(
                            "Failed to load test case '{}' for verification: {}",
                            test_case_id,
                            e
                        );
                    }
                }
            }
        }

        Ok(verification_results)
    }

    fn print_summary(&self, stats: &OrchestratorStats) {
        println!("=== Execution Summary ===");
        println!("Total test cases: {}", stats.total_tests);
        println!("Completed: {}", stats.completed_tests);
        println!("Passed: {} ({}%)", stats.passed_tests, stats.success_rate() as u32);
        println!("Failed: {}", stats.failed_tests);
        println!("Total attempts: {}", stats.total_attempts);
        println!(
            "Total time: {:.2}s",
            stats.elapsed_time_ms as f64 / 1000.0
        );
    }

    pub fn generate_execution_report(
        &self,
        results: &[TestExecutionResult],
        output_path: &Path,
    ) -> Result<()> {
        let mut report = String::new();
        report.push_str("# Test Execution Report\n\n");
        report.push_str(&format!("Generated at: {}\n\n", Utc::now().to_rfc3339()));

        let total = results.len();
        let passed = results.iter().filter(|r| r.success).count();
        let failed = total - passed;
        let total_attempts: usize = results.iter().map(|r| r.attempts).sum();
        let total_duration: f64 = results.iter().map(|r| r.duration_s).sum();

        report.push_str("## Summary\n\n");
        report.push_str(&format!("- **Total Tests**: {}\n", total));
        report.push_str(&format!("- **Passed**: {} ({:.1}%)\n", passed, (passed as f64 / total as f64) * 100.0));
        report.push_str(&format!("- **Failed**: {} ({:.1}%)\n", failed, (failed as f64 / total as f64) * 100.0));
        report.push_str(&format!("- **Total Attempts**: {}\n", total_attempts));
        report.push_str(&format!("- **Total Duration**: {:.2}s\n\n", total_duration));

        report.push_str("## Test Results\n\n");
        report.push_str("| Test Case ID | Status | Duration | Attempts |\n");
        report.push_str("|--------------|--------|----------|----------|\n");

        for result in results {
            let status = if result.success { "✓ PASS" } else { "✗ FAIL" };
            report.push_str(&format!(
                "| {} | {} | {}ms | {} |\n",
                result.test_case_id, status, result.duration_s, result.attempts
            ));
        }

        if failed > 0 {
            report.push_str("\n## Failed Tests\n\n");
            for result in results.iter().filter(|r| !r.success) {
                report.push_str(&format!("### {}\n\n", result.test_case_id));
                report.push_str(&format!("- **Attempts**: {}\n", result.attempts));
                report.push_str(&format!("- **Duration**: {}ms\n", result.duration_s));
                if let Some(error) = &result.error_message {
                    report.push_str(&format!("- **Error**: {}\n", error));
                }
                report.push_str("\n");
            }
        }

        fs::write(output_path, report).context(format!(
            "Failed to write report to {}",
            output_path.display()
        ))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Expected, Step, TestSequence, Verification};
    use tempfile::TempDir;

    fn create_simple_test_case(id: &str) -> TestCase {
        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            id.to_string(),
            format!("Test case {}", id),
        );

        let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Sequence 1".to_string());
        let step = Step {
            step: 1,
            manual: None,
            description: "Echo test".to_string(),
            command: "echo 'hello'".to_string(),
            expected: Expected {
                success: Some(true),
                result: "0".to_string(),
                output: "hello".to_string(),
            },
            verification: Verification {
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "echo \"$COMMAND_OUTPUT\" | grep -q 'hello'".to_string(),
            },
        };
        sequence.steps.push(step);
        test_case.test_sequences.push(sequence);

        test_case
    }

    #[test]
    fn test_retry_policy_creation() {
        let no_retry = RetryPolicy::no_retry();
        assert_eq!(no_retry.strategy, RetryStrategy::NoRetry);

        let fixed = RetryPolicy::fixed_retries(3);
        assert_eq!(fixed.strategy, RetryStrategy::FixedRetries { max_attempts: 3 });

        let exponential = RetryPolicy::exponential_backoff(5, 100);
        assert_eq!(
            exponential.strategy,
            RetryStrategy::ExponentialBackoff {
                max_attempts: 5,
                base_delay_ms: 100
            }
        );
    }

    #[test]
    fn test_worker_config() {
        let config = WorkerConfig::new(8);
        assert_eq!(config.num_workers, 8);
        assert_eq!(config.retry_policy.strategy, RetryStrategy::NoRetry);

        let config_with_retry = WorkerConfig::new(4).with_retry_policy(RetryPolicy::fixed_retries(2));
        assert_eq!(config_with_retry.num_workers, 4);
        assert_eq!(
            config_with_retry.retry_policy.strategy,
            RetryStrategy::FixedRetries { max_attempts: 2 }
        );
    }

    #[test]
    fn test_orchestrator_stats() {
        let mut stats = OrchestratorStats::new(10);
        assert_eq!(stats.total_tests, 10);
        assert_eq!(stats.completed_tests, 0);
        assert_eq!(stats.progress_percentage(), 0.0);

        stats.completed_tests = 5;
        stats.passed_tests = 3;
        stats.failed_tests = 2;
        assert_eq!(stats.progress_percentage(), 50.0);
        assert_eq!(stats.success_rate(), 60.0);
    }

    #[test]
    fn test_orchestrator_creation() {
        let temp_dir = TempDir::new().unwrap();
        let test_case_storage = TestCaseStorage::new(temp_dir.path()).unwrap();
        let test_run_storage = TestRunStorage::new(temp_dir.path().join("runs")).unwrap();
        let output_dir = temp_dir.path().join("output");

        let orchestrator =
            TestOrchestrator::new(test_case_storage, test_run_storage, output_dir.clone()).unwrap();

        assert!(output_dir.exists());
    }

    #[test]
    fn test_select_test_cases() {
        let temp_dir = TempDir::new().unwrap();
        let test_case_storage = TestCaseStorage::new(temp_dir.path()).unwrap();
        let test_run_storage = TestRunStorage::new(temp_dir.path().join("runs")).unwrap();
        let output_dir = temp_dir.path().join("output");

        let test_case1 = create_simple_test_case("TC001");
        let test_case2 = create_simple_test_case("TC002");

        test_case_storage.save_test_case(&test_case1).unwrap();
        test_case_storage.save_test_case(&test_case2).unwrap();

        let orchestrator =
            TestOrchestrator::new(test_case_storage, test_run_storage, output_dir).unwrap();

        let selected = orchestrator
            .select_test_cases(vec!["TC001".to_string(), "TC002".to_string()])
            .unwrap();

        assert_eq!(selected.len(), 2);
        assert_eq!(selected[0].id, "TC001");
        assert_eq!(selected[1].id, "TC002");
    }

    #[test]
    fn test_select_all_test_cases() {
        let temp_dir = TempDir::new().unwrap();
        let test_case_storage = TestCaseStorage::new(temp_dir.path()).unwrap();
        let test_run_storage = TestRunStorage::new(temp_dir.path().join("runs")).unwrap();
        let output_dir = temp_dir.path().join("output");

        let test_case1 = create_simple_test_case("TC001");
        let test_case2 = create_simple_test_case("TC002");
        let test_case3 = create_simple_test_case("TC003");

        test_case_storage.save_test_case(&test_case1).unwrap();
        test_case_storage.save_test_case(&test_case2).unwrap();
        test_case_storage.save_test_case(&test_case3).unwrap();

        let orchestrator =
            TestOrchestrator::new(test_case_storage, test_run_storage, output_dir).unwrap();

        let all_tests = orchestrator.select_all_test_cases().unwrap();
        assert_eq!(all_tests.len(), 3);
    }
}
