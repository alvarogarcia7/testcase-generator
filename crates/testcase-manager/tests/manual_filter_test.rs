use anyhow::Result;
use std::fs;
use tempfile::TempDir;
use testcase_manager::{
    orchestrator::TestOrchestrator, TestCase, TestCaseFilter, TestCaseFilterer, TestCaseStorage,
    TestExecutor, TestRunStorage, TestSequence,
};

fn create_test_case_with_manual_steps(
    requirement: &str,
    item: i64,
    tc: i64,
    id: &str,
    description: &str,
    manual_step_count: usize,
    automated_step_count: usize,
) -> TestCase {
    let mut test_case = TestCase::new(
        requirement.to_string(),
        item,
        tc,
        id.to_string(),
        description.to_string(),
    );

    let mut sequence = TestSequence::new(1, "Test Sequence".to_string(), "Test".to_string());

    for i in 1..=manual_step_count {
        let mut step = testcase_manager::Step::new(
            i as i64,
            format!("Manual step {}", i),
            format!("manual_command_{}", i),
            "0".to_string(),
            "success".to_string(),
        );
        step.manual = Some(true);
        sequence.steps.push(step);
    }

    for i in 1..=automated_step_count {
        let step = testcase_manager::Step::new(
            (manual_step_count + i) as i64,
            format!("Automated step {}", i),
            format!("automated_command_{}", i),
            "0".to_string(),
            "success".to_string(),
        );
        sequence.steps.push(step);
    }

    test_case.test_sequences.push(sequence);
    test_case
}

#[test]
fn test_list_test_cases_with_manual_steps() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let storage = TestCaseStorage::new(temp_dir.path())?;

    let tc1 = create_test_case_with_manual_steps("REQ001", 1, 1, "TC001", "Manual only", 2, 0);
    let tc2 = create_test_case_with_manual_steps("REQ002", 1, 2, "TC002", "Automated only", 0, 3);
    let tc3 = create_test_case_with_manual_steps("REQ003", 1, 3, "TC003", "Mixed", 1, 2);

    storage.save_test_case(&tc1)?;
    storage.save_test_case(&tc2)?;
    storage.save_test_case(&tc3)?;

    let all_test_cases = storage.load_all_test_cases()?;
    assert_eq!(all_test_cases.len(), 3);

    let filterer = TestCaseFilterer::new();

    let manual_only =
        filterer.filter_test_cases(all_test_cases.clone(), TestCaseFilter::ManualOnly);
    assert_eq!(manual_only.len(), 2);
    assert!(manual_only.iter().any(|tc| tc.id == "TC001"));
    assert!(manual_only.iter().any(|tc| tc.id == "TC003"));

    let automated_only =
        filterer.filter_test_cases(all_test_cases.clone(), TestCaseFilter::AutomatedOnly);
    assert_eq!(automated_only.len(), 2);
    assert!(automated_only.iter().any(|tc| tc.id == "TC002"));
    assert!(automated_only.iter().any(|tc| tc.id == "TC003"));

    let all = filterer.filter_test_cases(all_test_cases, TestCaseFilter::All);
    assert_eq!(all.len(), 3);

    Ok(())
}

#[test]
fn test_filter_manual_only_test_cases() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let storage = TestCaseStorage::new(temp_dir.path())?;

    let tc1 =
        create_test_case_with_manual_steps("REQ001", 1, 1, "TC_MANUAL_001", "Fully manual", 3, 0);
    let tc2 =
        create_test_case_with_manual_steps("REQ002", 1, 2, "TC_AUTO_001", "Fully automated", 0, 3);
    let tc3 =
        create_test_case_with_manual_steps("REQ003", 1, 3, "TC_MIXED_001", "Mixed steps", 2, 2);

    storage.save_test_case(&tc1)?;
    storage.save_test_case(&tc2)?;
    storage.save_test_case(&tc3)?;

    let all_test_cases = storage.load_all_test_cases()?;
    let filterer = TestCaseFilterer::new();

    let manual_only = filterer.filter_test_cases(all_test_cases, TestCaseFilter::ManualOnly);

    assert_eq!(manual_only.len(), 2);

    let manual_ids: Vec<String> = manual_only.iter().map(|tc| tc.id.clone()).collect();
    assert!(manual_ids.contains(&"TC_MANUAL_001".to_string()));
    assert!(manual_ids.contains(&"TC_MIXED_001".to_string()));
    assert!(!manual_ids.contains(&"TC_AUTO_001".to_string()));

    for tc in &manual_only {
        assert!(
            tc.has_manual_steps(),
            "Test case {} should have manual steps",
            tc.id
        );
    }

    Ok(())
}

#[test]
fn test_filter_automated_only_test_cases() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let storage = TestCaseStorage::new(temp_dir.path())?;

    let tc1 =
        create_test_case_with_manual_steps("REQ001", 1, 1, "TC_MANUAL_ONLY", "Manual only", 3, 0);
    let tc2 =
        create_test_case_with_manual_steps("REQ002", 1, 2, "TC_AUTO_ONLY", "Automated only", 0, 3);
    let tc3 = create_test_case_with_manual_steps("REQ003", 1, 3, "TC_MIXED", "Mixed steps", 2, 2);

    storage.save_test_case(&tc1)?;
    storage.save_test_case(&tc2)?;
    storage.save_test_case(&tc3)?;

    let all_test_cases = storage.load_all_test_cases()?;
    let filterer = TestCaseFilterer::new();

    let automated_only = filterer.filter_test_cases(all_test_cases, TestCaseFilter::AutomatedOnly);

    assert_eq!(automated_only.len(), 2);

    let auto_ids: Vec<String> = automated_only.iter().map(|tc| tc.id.clone()).collect();
    assert!(auto_ids.contains(&"TC_AUTO_ONLY".to_string()));
    assert!(auto_ids.contains(&"TC_MIXED".to_string()));
    assert!(!auto_ids.contains(&"TC_MANUAL_ONLY".to_string()));

    for tc in &automated_only {
        assert!(
            tc.has_automated_steps(),
            "Test case {} should have automated steps",
            tc.id
        );
    }

    Ok(())
}

#[test]
fn test_orchestrator_execution_with_manual_filter() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_case_storage = TestCaseStorage::new(temp_dir.path())?;
    let test_run_storage = TestRunStorage::new(temp_dir.path().join("test-runs"))?;
    let output_dir = temp_dir.path().join("output");

    let tc1 =
        create_test_case_with_manual_steps("REQ001", 1, 1, "TC_MANUAL_001", "Manual test", 2, 0);
    let tc2 = create_test_case_with_manual_steps("REQ002", 1, 2, "TC_AUTO_001", "Auto test", 0, 2);
    let tc3 =
        create_test_case_with_manual_steps("REQ003", 1, 3, "TC_MIXED_001", "Mixed test", 1, 1);

    test_case_storage.save_test_case(&tc1)?;
    test_case_storage.save_test_case(&tc2)?;
    test_case_storage.save_test_case(&tc3)?;

    let orchestrator = TestOrchestrator::new(test_case_storage, test_run_storage, output_dir)?;

    let all_test_cases = orchestrator.select_all_test_cases()?;
    assert_eq!(all_test_cases.len(), 3);

    let filterer = TestCaseFilterer::new();
    let manual_test_cases =
        filterer.filter_test_cases(all_test_cases.clone(), TestCaseFilter::ManualOnly);

    assert_eq!(manual_test_cases.len(), 2);
    assert!(manual_test_cases.iter().any(|tc| tc.id == "TC_MANUAL_001"));
    assert!(manual_test_cases.iter().any(|tc| tc.id == "TC_MIXED_001"));

    let automated_test_cases =
        filterer.filter_test_cases(all_test_cases, TestCaseFilter::AutomatedOnly);

    assert_eq!(automated_test_cases.len(), 2);
    assert!(automated_test_cases.iter().any(|tc| tc.id == "TC_AUTO_001"));
    assert!(automated_test_cases
        .iter()
        .any(|tc| tc.id == "TC_MIXED_001"));

    Ok(())
}

#[test]
fn test_statistics_display_with_manual_steps() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let storage = TestCaseStorage::new(temp_dir.path())?;

    let tc1 = create_test_case_with_manual_steps("REQ001", 1, 1, "TC001", "Test 1", 3, 2);
    let tc2 = create_test_case_with_manual_steps("REQ002", 1, 2, "TC002", "Test 2", 0, 5);
    let tc3 = create_test_case_with_manual_steps("REQ003", 1, 3, "TC003", "Test 3", 2, 0);
    let tc4 = create_test_case_with_manual_steps("REQ004", 1, 4, "TC004", "Test 4", 1, 3);

    storage.save_test_case(&tc1)?;
    storage.save_test_case(&tc2)?;
    storage.save_test_case(&tc3)?;
    storage.save_test_case(&tc4)?;

    let all_test_cases = storage.load_all_test_cases()?;
    assert_eq!(all_test_cases.len(), 4);

    let filterer = TestCaseFilterer::new();
    let filtered = filterer.filter_test_cases(all_test_cases.clone(), TestCaseFilter::All);

    let total_count = filtered.len();
    let manual_count = filtered.iter().filter(|tc| tc.has_manual_steps()).count();
    let automated_count = filtered
        .iter()
        .filter(|tc| tc.has_automated_steps())
        .count();
    let total_manual_steps: usize = filtered.iter().map(|tc| tc.get_manual_step_count()).sum();

    assert_eq!(total_count, 4);
    assert_eq!(manual_count, 3);
    assert_eq!(automated_count, 3);
    assert_eq!(total_manual_steps, 6);

    let tc1_manual_count = all_test_cases
        .iter()
        .find(|tc| tc.id == "TC001")
        .map(|tc| tc.get_manual_step_count())
        .unwrap();
    assert_eq!(tc1_manual_count, 3);

    let tc2_manual_count = all_test_cases
        .iter()
        .find(|tc| tc.id == "TC002")
        .map(|tc| tc.get_manual_step_count())
        .unwrap();
    assert_eq!(tc2_manual_count, 0);

    Ok(())
}

#[test]
fn test_manual_step_indicator_in_list() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let storage = TestCaseStorage::new(temp_dir.path())?;

    let tc1 =
        create_test_case_with_manual_steps("REQ001", 1, 1, "TC_WITH_MANUAL", "Has manual", 2, 1);
    let tc2 = create_test_case_with_manual_steps("REQ002", 1, 2, "TC_NO_MANUAL", "No manual", 0, 3);

    storage.save_test_case(&tc1)?;
    storage.save_test_case(&tc2)?;

    let test_cases = storage.load_all_test_cases()?;

    for test_case in &test_cases {
        let manual_step_count = test_case.get_manual_step_count();
        if test_case.id == "TC_WITH_MANUAL" {
            assert_eq!(manual_step_count, 2);
            assert!(test_case.has_manual_steps());
        } else if test_case.id == "TC_NO_MANUAL" {
            assert_eq!(manual_step_count, 0);
            assert!(!test_case.has_manual_steps());
        }
    }

    Ok(())
}

#[test]
fn test_executor_with_manual_steps() -> Result<()> {
    let tc = create_test_case_with_manual_steps("REQ001", 1, 1, "TC_MANUAL", "Manual test", 2, 1);

    assert_eq!(tc.get_manual_step_count(), 2);
    assert!(tc.has_manual_steps());
    assert!(tc.has_automated_steps());

    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&tc);

    assert!(script.contains("Manual step 1"));
    assert!(script.contains("Manual step 2"));
    assert!(script.contains("Automated step 1"));

    assert!(script.contains("INFO: This is a manual step"));

    Ok(())
}

#[test]
fn test_filter_edge_cases() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let storage = TestCaseStorage::new(temp_dir.path())?;

    let empty_tc = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC_EMPTY".to_string(),
        "Empty test".to_string(),
    );

    storage.save_test_case(&empty_tc)?;

    let all_test_cases = storage.load_all_test_cases()?;
    let filterer = TestCaseFilterer::new();

    let manual_only =
        filterer.filter_test_cases(all_test_cases.clone(), TestCaseFilter::ManualOnly);
    assert_eq!(manual_only.len(), 0);

    let automated_only =
        filterer.filter_test_cases(all_test_cases.clone(), TestCaseFilter::AutomatedOnly);
    assert_eq!(automated_only.len(), 0);

    let all = filterer.filter_test_cases(all_test_cases, TestCaseFilter::All);
    assert_eq!(all.len(), 1);

    Ok(())
}

#[test]
fn test_manual_steps_across_multiple_sequences() -> Result<()> {
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC_MULTI_SEQ".to_string(),
        "Multi-sequence test".to_string(),
    );

    let mut seq1 = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
    let mut manual_step1 = testcase_manager::Step::new(
        1,
        "Manual step in seq1".to_string(),
        "manual_cmd1".to_string(),
        "0".to_string(),
        "success".to_string(),
    );
    manual_step1.manual = Some(true);
    seq1.steps.push(manual_step1);

    let auto_step1 = testcase_manager::Step::new(
        2,
        "Auto step in seq1".to_string(),
        "auto_cmd1".to_string(),
        "0".to_string(),
        "success".to_string(),
    );
    seq1.steps.push(auto_step1);

    let mut seq2 = TestSequence::new(2, "Seq2".to_string(), "Second sequence".to_string());
    let mut manual_step2 = testcase_manager::Step::new(
        1,
        "Manual step in seq2".to_string(),
        "manual_cmd2".to_string(),
        "0".to_string(),
        "success".to_string(),
    );
    manual_step2.manual = Some(true);
    seq2.steps.push(manual_step2);

    let mut manual_step3 = testcase_manager::Step::new(
        2,
        "Another manual step in seq2".to_string(),
        "manual_cmd3".to_string(),
        "0".to_string(),
        "success".to_string(),
    );
    manual_step3.manual = Some(true);
    seq2.steps.push(manual_step3);

    test_case.test_sequences.push(seq1);
    test_case.test_sequences.push(seq2);

    assert_eq!(test_case.get_manual_step_count(), 3);
    assert!(test_case.has_manual_steps());
    assert!(test_case.has_automated_steps());

    Ok(())
}

#[test]
fn test_filter_with_explicit_false_manual_field() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let storage = TestCaseStorage::new(temp_dir.path())?;

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC_EXPLICIT".to_string(),
        "Explicit false test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());

    let mut step1 = testcase_manager::Step::new(
        1,
        "Step with explicit false".to_string(),
        "cmd1".to_string(),
        "0".to_string(),
        "success".to_string(),
    );
    step1.manual = Some(false);
    sequence.steps.push(step1);

    let mut step2 = testcase_manager::Step::new(
        2,
        "Step without manual field".to_string(),
        "cmd2".to_string(),
        "0".to_string(),
        "success".to_string(),
    );
    step2.manual = None;
    sequence.steps.push(step2);

    test_case.test_sequences.push(sequence);

    assert_eq!(test_case.get_manual_step_count(), 0);
    assert!(!test_case.has_manual_steps());
    assert!(test_case.has_automated_steps());

    storage.save_test_case(&test_case)?;

    let all_test_cases = storage.load_all_test_cases()?;
    let filterer = TestCaseFilterer::new();

    let manual_only =
        filterer.filter_test_cases(all_test_cases.clone(), TestCaseFilter::ManualOnly);
    assert_eq!(manual_only.len(), 0);

    let automated_only = filterer.filter_test_cases(all_test_cases, TestCaseFilter::AutomatedOnly);
    assert_eq!(automated_only.len(), 1);

    Ok(())
}

#[test]
fn test_statistics_with_filtered_results() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let storage = TestCaseStorage::new(temp_dir.path())?;

    let tc1 = create_test_case_with_manual_steps("REQ001", 1, 1, "TC001", "Test 1", 4, 1);
    let tc2 = create_test_case_with_manual_steps("REQ002", 1, 2, "TC002", "Test 2", 2, 3);
    let tc3 = create_test_case_with_manual_steps("REQ003", 1, 3, "TC003", "Test 3", 0, 5);
    let tc4 = create_test_case_with_manual_steps("REQ004", 1, 4, "TC004", "Test 4", 1, 0);

    storage.save_test_case(&tc1)?;
    storage.save_test_case(&tc2)?;
    storage.save_test_case(&tc3)?;
    storage.save_test_case(&tc4)?;

    let all_test_cases = storage.load_all_test_cases()?;
    let filterer = TestCaseFilterer::new();

    let manual_filtered =
        filterer.filter_test_cases(all_test_cases.clone(), TestCaseFilter::ManualOnly);
    let manual_total_steps: usize = manual_filtered
        .iter()
        .map(|tc| tc.get_manual_step_count())
        .sum();
    assert_eq!(manual_filtered.len(), 3);
    assert_eq!(manual_total_steps, 7);

    let automated_filtered =
        filterer.filter_test_cases(all_test_cases, TestCaseFilter::AutomatedOnly);
    assert_eq!(automated_filtered.len(), 3);

    let tc_with_both = automated_filtered
        .iter()
        .filter(|tc| tc.has_manual_steps() && tc.has_automated_steps())
        .count();
    assert_eq!(tc_with_both, 2);

    Ok(())
}

#[test]
fn test_yaml_serialization_preserves_manual_field() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let storage = TestCaseStorage::new(temp_dir.path())?;

    let tc = create_test_case_with_manual_steps("REQ001", 1, 1, "TC_YAML", "YAML test", 2, 1);

    let file_path = storage.save_test_case(&tc)?;

    let yaml_content = fs::read_to_string(&file_path)?;

    assert!(yaml_content.contains("manual: true"));

    let loaded_tc = storage.load_test_case_by_id("TC_YAML")?;

    assert_eq!(loaded_tc.get_manual_step_count(), 2);
    assert!(loaded_tc.has_manual_steps());

    for sequence in &loaded_tc.test_sequences {
        for step in &sequence.steps {
            if step.description.contains("Manual") {
                assert_eq!(step.manual, Some(true));
            } else if step.description.contains("Automated") {
                assert_ne!(step.manual, Some(true));
            }
        }
    }

    Ok(())
}

#[test]
fn test_filter_integration_with_storage() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let storage = TestCaseStorage::new(temp_dir.path())?;

    for i in 1..=10 {
        let manual_count = if i % 3 == 0 { i % 4 } else { 0 };
        let auto_count = if i % 2 == 0 { i % 3 + 1 } else { 0 };

        let tc = create_test_case_with_manual_steps(
            &format!("REQ{:03}", i),
            1,
            i as i64,
            &format!("TC{:03}", i),
            &format!("Test case {}", i),
            manual_count,
            auto_count,
        );
        storage.save_test_case(&tc)?;
    }

    let all_test_cases = storage.load_all_test_cases()?;
    assert_eq!(all_test_cases.len(), 10);

    let filterer = TestCaseFilterer::new();

    let manual_only =
        filterer.filter_test_cases(all_test_cases.clone(), TestCaseFilter::ManualOnly);
    let automated_only =
        filterer.filter_test_cases(all_test_cases.clone(), TestCaseFilter::AutomatedOnly);
    let all = filterer.filter_test_cases(all_test_cases, TestCaseFilter::All);

    assert!(manual_only.len() <= all.len());
    assert!(automated_only.len() <= all.len());

    for tc in &manual_only {
        assert!(tc.has_manual_steps());
    }

    for tc in &automated_only {
        assert!(tc.has_automated_steps());
    }

    Ok(())
}

#[test]
fn test_mixed_test_case_appears_in_both_filters() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let storage = TestCaseStorage::new(temp_dir.path())?;

    let mixed_tc =
        create_test_case_with_manual_steps("REQ001", 1, 1, "TC_MIXED", "Mixed steps test", 3, 3);

    storage.save_test_case(&mixed_tc)?;

    let all_test_cases = storage.load_all_test_cases()?;
    let filterer = TestCaseFilterer::new();

    let manual_only =
        filterer.filter_test_cases(all_test_cases.clone(), TestCaseFilter::ManualOnly);
    let automated_only = filterer.filter_test_cases(all_test_cases, TestCaseFilter::AutomatedOnly);

    assert_eq!(manual_only.len(), 1);
    assert_eq!(automated_only.len(), 1);

    assert_eq!(manual_only[0].id, "TC_MIXED");
    assert_eq!(automated_only[0].id, "TC_MIXED");

    Ok(())
}

#[test]
fn test_orchestrator_result_paths_with_manual_filter() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_case_storage = TestCaseStorage::new(temp_dir.path())?;
    let test_run_storage = TestRunStorage::new(temp_dir.path().join("test-runs"))?;
    let output_dir = temp_dir.path().join("output");

    let tc_manual =
        create_test_case_with_manual_steps("REQ001", 1, 1, "TC_MANUAL", "Manual test", 3, 0);
    let tc_auto =
        create_test_case_with_manual_steps("REQ002", 1, 2, "TC_AUTO", "Automated test", 0, 3);

    test_case_storage.save_test_case(&tc_manual)?;
    test_case_storage.save_test_case(&tc_auto)?;

    let orchestrator =
        TestOrchestrator::new(test_case_storage, test_run_storage, output_dir.clone())?;

    let all_test_cases = orchestrator.select_all_test_cases()?;
    let filterer = TestCaseFilterer::new();

    let manual_test_cases =
        filterer.filter_test_cases(all_test_cases.clone(), TestCaseFilter::ManualOnly);
    assert_eq!(manual_test_cases.len(), 1);
    assert_eq!(manual_test_cases[0].id, "TC_MANUAL");

    let automated_test_cases =
        filterer.filter_test_cases(all_test_cases, TestCaseFilter::AutomatedOnly);
    assert_eq!(automated_test_cases.len(), 1);
    assert_eq!(automated_test_cases[0].id, "TC_AUTO");

    Ok(())
}
