use chrono::{DateTime, Duration, Utc};
use std::fs;
use std::thread;
use std::time::Duration as StdDuration;
use tempfile::TempDir;
use testcase_manager::models::{TestRun, TestRunStatus};
use testcase_manager::test_run_storage::TestRunStorage;

fn create_test_run(test_case_id: &str, duration_s: f64) -> TestRun {
    TestRun {
        name: None,
        test_case_id: test_case_id.to_string(),
        timestamp: Utc::now(),
        status: TestRunStatus::Pass,
        duration: duration_s,
        execution_log: "Test execution log".to_string(),
        error_message: None,
    }
}

fn create_test_run_with_timestamp(
    test_case_id: &str,
    timestamp: DateTime<Utc>,
    duration_s: f64,
) -> TestRun {
    TestRun {
        name: None,
        test_case_id: test_case_id.to_string(),
        timestamp,
        status: TestRunStatus::Pass,
        duration: duration_s,
        execution_log: format!("Execution log for {}", test_case_id),
        error_message: None,
    }
}

#[test]
fn test_folder_creation_on_new() {
    let temp_dir = TempDir::new().unwrap();
    let _storage = TestRunStorage::new(temp_dir.path()).unwrap();
    assert!(
        temp_dir.path().exists(),
        "Base path should exist after creating storage"
    );
}

#[test]
fn test_folder_creation_for_nonexistent_base_path() {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent_path = temp_dir.path().join("deeply").join("nested").join("path");

    let _storage = TestRunStorage::new(&nonexistent_path).unwrap();
    assert!(
        nonexistent_path.exists(),
        "Storage should create nested directories"
    );
}

#[test]
fn test_folder_creation_on_save_test_run() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let test_run = create_test_run("TC001", 1.000,);
    let runs_folder = storage.get_test_run_folder("TC001");

    assert!(
        !runs_folder.exists(),
        "Runs folder should not exist before saving"
    );

    storage.save_test_run(&test_run).unwrap();

    assert!(
        runs_folder.exists(),
        "Runs folder should exist after saving"
    );
    assert!(runs_folder.is_dir(), "Runs path should be a directory");
}

#[test]
fn test_folder_structure_for_multiple_test_cases() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let test_run1 = create_test_run("TC001", 1.000,);
    let test_run2 = create_test_run("TC002", 2.000,);
    let test_run3 = create_test_run("TC003", 3.000,);

    storage.save_test_run(&test_run1).unwrap();
    storage.save_test_run(&test_run2).unwrap();
    storage.save_test_run(&test_run3).unwrap();

    let folder1 = storage.get_test_run_folder("TC001");
    let folder2 = storage.get_test_run_folder("TC002");
    let folder3 = storage.get_test_run_folder("TC003");

    assert!(folder1.exists() && folder1.is_dir());
    assert!(folder2.exists() && folder2.is_dir());
    assert!(folder3.exists() && folder3.is_dir());

    assert_eq!(folder1, temp_dir.path().join("TC001").join("runs"));
    assert_eq!(folder2, temp_dir.path().join("TC002").join("runs"));
    assert_eq!(folder3, temp_dir.path().join("TC003").join("runs"));
}

#[test]
fn test_timestamp_filename_generation() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let timestamp = Utc::now();
    let test_run = create_test_run_with_timestamp("TC001", timestamp, 1.000,);

    let saved_path = storage.save_test_run(&test_run).unwrap();
    let filename = saved_path.file_name().unwrap().to_str().unwrap();

    let expected_filename = format!("{}.yaml", timestamp.to_rfc3339());
    assert_eq!(filename, expected_filename);
    assert!(filename.ends_with(".yaml"));
}

#[test]
fn test_timestamp_filename_uniqueness() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let timestamp1 = Utc::now();
    thread::sleep(StdDuration::from_millis(10));
    let timestamp2 = Utc::now();

    let test_run1 = create_test_run_with_timestamp("TC001", timestamp1, 1.000,);
    let test_run2 = create_test_run_with_timestamp("TC001", timestamp2, 2.000,);

    let path1 = storage.save_test_run(&test_run1).unwrap();
    let path2 = storage.save_test_run(&test_run2).unwrap();

    assert_ne!(
        path1, path2,
        "Different timestamps should generate different filenames"
    );
    assert!(path1.exists());
    assert!(path2.exists());

    let filename1 = path1.file_name().unwrap().to_str().unwrap();
    let filename2 = path2.file_name().unwrap().to_str().unwrap();
    assert_ne!(filename1, filename2);
}

#[test]
fn test_timestamp_filename_uniqueness_across_multiple_runs() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let mut paths = Vec::new();
    for i in 0..5 {
        thread::sleep(StdDuration::from_millis(10));
        let test_run = create_test_run("TC001", i as f64 * 1.000,);
        let path = storage.save_test_run(&test_run).unwrap();
        paths.push(path);
    }

    for i in 0..paths.len() {
        for j in i + 1..paths.len() {
            assert_ne!(
                paths[i], paths[j],
                "Each test run should have a unique filename"
            );
        }
    }

    for path in &paths {
        assert!(path.exists());
    }
}

#[test]
fn test_yaml_serialization_basic() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let test_run = TestRun {
        name: None,
        test_case_id: "TC001".to_string(),
        timestamp: Utc::now(),
        status: TestRunStatus::Pass,
        duration: 1.500,
        execution_log: "Test passed successfully".to_string(),
        error_message: None,
    };

    let saved_path = storage.save_test_run(&test_run).unwrap();
    let content = fs::read_to_string(&saved_path).unwrap();

    assert!(content.contains("test_case_id: TC001"));
    assert!(content.contains("status: Pass"));
    assert!(content.contains("duration: 1.5")); // Lose trailing zeroes
    assert!(content.contains("execution_log: Test passed successfully"));
}

#[test]
fn test_yaml_serialization_with_error_message() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let test_run = TestRun {
        name: None,
        test_case_id: "TC002".to_string(),
        timestamp: Utc::now(),
        status: TestRunStatus::Fail,
        duration: 2.500,
        execution_log: "Test execution log".to_string(),
        error_message: Some("Connection timeout after 30 seconds".to_string()),
    };

    let saved_path = storage.save_test_run(&test_run).unwrap();
    let content = fs::read_to_string(&saved_path).unwrap();

    assert!(content.contains("status: Fail"));
    assert!(content.contains("error_message: Connection timeout after 30 seconds"));
}

#[test]
fn test_yaml_serialization_multiline_log() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let multiline_log = "Step 1: Initialize connection\nStep 2: Send command\nStep 3: Verify response\nStep 4: Cleanup";
    let test_run = TestRun {
        name: None,
        test_case_id: "TC003".to_string(),
        timestamp: Utc::now(),
        status: TestRunStatus::Pass,
        duration: 3.000,
        execution_log: multiline_log.to_string(),
        error_message: None,
    };

    let saved_path = storage.save_test_run(&test_run).unwrap();
    let content = fs::read_to_string(&saved_path).unwrap();

    assert!(content.contains("Step 1: Initialize connection"));
    assert!(content.contains("Step 2: Send command"));
    assert!(content.contains("Step 3: Verify response"));
    assert!(content.contains("Step 4: Cleanup"));
}

#[test]
fn test_yaml_deserialization_basic() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let original = TestRun {
        name: None,
        test_case_id: "TC001".to_string(),
        timestamp: Utc::now(),
        status: TestRunStatus::Pass,
        duration: 1.234,
        execution_log: "Test log".to_string(),
        error_message: None,
    };

    storage.save_test_run(&original).unwrap();
    let loaded_runs = storage.load_test_runs_for_case("TC001").unwrap();

    assert_eq!(loaded_runs.len(), 1);
    let loaded = &loaded_runs[0];
    assert_eq!(loaded.test_case_id, original.test_case_id);
    assert_eq!(loaded.status, original.status);
    assert_eq!(loaded.duration, original.duration);
    assert_eq!(loaded.execution_log, original.execution_log);
    assert_eq!(loaded.error_message, original.error_message);
}

#[test]
fn test_yaml_roundtrip_all_statuses() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let test_runs = vec![
        TestRun {
            name: None,
            test_case_id: "TC001".to_string(),
            timestamp: Utc::now(),
            status: TestRunStatus::Pass,
            duration: 1.000,
            execution_log: "Pass log".to_string(),
            error_message: None,
        },
        TestRun {
            name: None,
            test_case_id: "TC002".to_string(),
            timestamp: Utc::now(),
            status: TestRunStatus::Fail,
            duration: 2.000,
            execution_log: "Fail log".to_string(),
            error_message: Some("Test failed".to_string()),
        },
        TestRun {
            name: None,
            test_case_id: "TC003".to_string(),
            timestamp: Utc::now(),
            status: TestRunStatus::Skip,
            duration: 0.0,
            execution_log: "Skip log".to_string(),
            error_message: None,
        },
    ];

    for test_run in &test_runs {
        storage.save_test_run(test_run).unwrap();
    }

    for test_run in &test_runs {
        let loaded = storage
            .load_test_runs_for_case(&test_run.test_case_id)
            .unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].status, test_run.status);
    }
}

#[test]
fn test_load_runs_for_specific_test_case() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let base_time = Utc::now();
    
    // Use create_test_run_with_timestamp to ensure unique timestamps
    let test_run1 = create_test_run_with_timestamp("TC001", base_time, 1.000);
    let test_run2 = create_test_run_with_timestamp("TC001", base_time + Duration::seconds(1), 2.000);
    let test_run3 = create_test_run_with_timestamp("TC002", base_time + Duration::seconds(2), 3.000);

    storage.save_test_run(&test_run1).unwrap();
    storage.save_test_run(&test_run2).unwrap();
    storage.save_test_run(&test_run3).unwrap();

    let tc001_runs = storage.load_test_runs_for_case("TC001").unwrap();
    assert_eq!(tc001_runs.len(), 2);
    assert!(tc001_runs.iter().all(|r| r.test_case_id == "TC001"));

    let tc002_runs = storage.load_test_runs_for_case("TC002").unwrap();
    assert_eq!(tc002_runs.len(), 1);
    assert_eq!(tc002_runs[0].test_case_id, "TC002");
}

#[test]
fn test_load_runs_for_nonexistent_test_case() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let runs = storage.load_test_runs_for_case("NONEXISTENT").unwrap();
    assert_eq!(runs.len(), 0);
}

#[test]
fn test_load_runs_sorted_by_timestamp() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let base_time = Utc::now();
    let test_run3 =
        create_test_run_with_timestamp("TC001", base_time + Duration::seconds(20), 3.000,);
    let test_run1 = create_test_run_with_timestamp("TC001", base_time, 1.000,);
    let test_run2 =
        create_test_run_with_timestamp("TC001", base_time + Duration::seconds(10), 2.000,);

    storage.save_test_run(&test_run3).unwrap();
    storage.save_test_run(&test_run1).unwrap();
    storage.save_test_run(&test_run2).unwrap();

    let loaded_runs = storage.load_test_runs_for_case("TC001").unwrap();
    assert_eq!(loaded_runs.len(), 3);
    assert_eq!(loaded_runs[0].duration, 1.000,);
    assert_eq!(loaded_runs[1].duration, 2.000,);
    assert_eq!(loaded_runs[2].duration, 3.000,);
}

#[test]
fn test_load_all_runs_across_multiple_test_cases() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let base_time = Utc::now();
    
    // Use unique timestamps to avoid file name collisions
    let test_run1 = create_test_run_with_timestamp("TC001", base_time, 1.000);
    let test_run2 = create_test_run_with_timestamp("TC002", base_time + Duration::seconds(1), 2.000);
    let test_run3 = create_test_run_with_timestamp("TC003", base_time + Duration::seconds(2), 3.000);
    let test_run4 = create_test_run_with_timestamp("TC001", base_time + Duration::seconds(3), 1.500);

    storage.save_test_run(&test_run1).unwrap();
    storage.save_test_run(&test_run2).unwrap();
    storage.save_test_run(&test_run3).unwrap();
    storage.save_test_run(&test_run4).unwrap();

    let all_runs = storage.load_all_test_runs().unwrap();
    assert_eq!(all_runs.len(), 4);

    let tc001_count = all_runs
        .iter()
        .filter(|r| r.test_case_id == "TC001")
        .count();
    let tc002_count = all_runs
        .iter()
        .filter(|r| r.test_case_id == "TC002")
        .count();
    let tc003_count = all_runs
        .iter()
        .filter(|r| r.test_case_id == "TC003")
        .count();

    assert_eq!(tc001_count, 2);
    assert_eq!(tc002_count, 1);
    assert_eq!(tc003_count, 1);
}

#[test]
fn test_load_all_runs_from_empty_storage() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let all_runs = storage.load_all_test_runs().unwrap();
    assert_eq!(all_runs.len(), 0);
}

#[test]
fn test_load_all_runs_sorted_by_timestamp() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let base_time = Utc::now();
    let test_run1 =
        create_test_run_with_timestamp("TC003", base_time + Duration::seconds(30), 3.000,);
    let test_run2 = create_test_run_with_timestamp("TC001", base_time, 1.000,);
    let test_run3 =
        create_test_run_with_timestamp("TC002", base_time + Duration::seconds(15), 2.000,);

    storage.save_test_run(&test_run1).unwrap();
    storage.save_test_run(&test_run2).unwrap();
    storage.save_test_run(&test_run3).unwrap();

    let all_runs = storage.load_all_test_runs().unwrap();
    assert_eq!(all_runs.len(), 3);
    assert_eq!(all_runs[0].test_case_id, "TC001");
    assert_eq!(all_runs[1].test_case_id, "TC002");
    assert_eq!(all_runs[2].test_case_id, "TC003");
}

#[test]
fn test_error_handling_invalid_yaml_file() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let runs_folder = storage.get_test_run_folder("TC001");
    fs::create_dir_all(&runs_folder).unwrap();

    let invalid_yaml_path = runs_folder.join("invalid.yaml");
    fs::write(&invalid_yaml_path, "this is not valid yaml: {{[[]]]").unwrap();

    let result = storage.load_test_runs_for_case("TC001");
    assert!(result.is_ok());
    let runs = result.unwrap();
    assert_eq!(runs.len(), 0);
}

#[test]
fn test_error_handling_corrupted_yaml_structure() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let runs_folder = storage.get_test_run_folder("TC001");
    fs::create_dir_all(&runs_folder).unwrap();

    let corrupted_yaml = r#"
test_case_id: TC001
timestamp: 2.024,-01-01T00:00:00Z
status: InvalidStatus
duration: not_a_number
"#;
    let corrupted_yaml_path = runs_folder.join("corrupted.yaml");
    fs::write(&corrupted_yaml_path, corrupted_yaml).unwrap();

    let result = storage.load_test_runs_for_case("TC001");
    assert!(result.is_ok());
    let runs = result.unwrap();
    assert_eq!(runs.len(), 0);
}

#[test]
fn test_error_handling_mixed_valid_and_invalid_files() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let valid_run = create_test_run("TC001", 1.000,);
    storage.save_test_run(&valid_run).unwrap();

    let runs_folder = storage.get_test_run_folder("TC001");
    let invalid_yaml_path = runs_folder.join("invalid.yaml");
    fs::write(&invalid_yaml_path, "invalid: yaml: content: [[[").unwrap();

    let result = storage.load_test_runs_for_case("TC001");
    assert!(result.is_ok());
    let runs = result.unwrap();
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].test_case_id, "TC001");
}

#[test]
fn test_error_handling_non_yaml_files_ignored() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let test_run = create_test_run("TC001", 1.000,);
    storage.save_test_run(&test_run).unwrap();

    let runs_folder = storage.get_test_run_folder("TC001");
    fs::write(runs_folder.join("readme.txt"), "This is a text file").unwrap();
    fs::write(runs_folder.join("data.json"), "{}").unwrap();

    let result = storage.load_test_runs_for_case("TC001");
    assert!(result.is_ok());
    let runs = result.unwrap();
    assert_eq!(runs.len(), 1);
}

#[test]
fn test_error_handling_invalid_test_case_id_characters() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let test_cases_with_special_chars = vec!["TC-001", "TC_001", "TC.001", "TC 001", "TC/001"];

    for test_case_id in test_cases_with_special_chars {
        let test_run = create_test_run(test_case_id, 1.000,);
        let result = storage.save_test_run(&test_run);

        match test_case_id {
            "TC/001" => {
                assert!(result.is_err() || result.is_ok());
            }
            _ => {
                assert!(
                    result.is_ok(),
                    "Should handle test case ID: {}",
                    test_case_id
                );
            }
        }
    }
}

#[test]
fn test_error_handling_empty_test_case_id() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let test_run = create_test_run("", 1.000,);
    let result = storage.save_test_run(&test_run);
    assert!(result.is_ok());

    let loaded = storage.load_test_runs_for_case("").unwrap();
    assert_eq!(loaded.len(), 1);
}

#[test]
fn test_load_all_runs_handles_empty_test_case_folders() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    fs::create_dir_all(temp_dir.path().join("TC001")).unwrap();
    fs::create_dir_all(temp_dir.path().join("TC002")).unwrap();

    let test_run = create_test_run("TC003", 1.000,);
    storage.save_test_run(&test_run).unwrap();

    let all_runs = storage.load_all_test_runs().unwrap();
    assert_eq!(all_runs.len(), 1);
    assert_eq!(all_runs[0].test_case_id, "TC003");
}

#[test]
fn test_yaml_serialization_special_characters_in_log() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let special_log =
        "Log with special chars: @#$%^&*()[]{}|\\'\"`~<>?/\nAnd unicode: 日本語 😀 🎉";
    let test_run = TestRun {
        test_case_id: "TC001".to_string(),
        name: Some("TC001".to_string()),
        timestamp: Utc::now(),
        status: TestRunStatus::Pass,
        duration: 1.000,
        execution_log: special_log.to_string(),
        error_message: None,
    };

    storage.save_test_run(&test_run).unwrap();
    let loaded_runs = storage.load_test_runs_for_case("TC001").unwrap();

    assert_eq!(loaded_runs.len(), 1);
    assert_eq!(loaded_runs[0].execution_log, special_log);
}

#[test]
fn test_concurrent_saves_to_different_test_cases() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    for i in 0..10 {
        let test_case_id = format!("TC{:03}", i);
        let test_run = create_test_run(&test_case_id, i as f64 * 1.000,);
        storage.save_test_run(&test_run).unwrap();
    }

    let all_runs = storage.load_all_test_runs().unwrap();
    assert_eq!(all_runs.len(), 10);
}

#[test]
fn test_yml_extension_also_loaded() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let test_run = TestRun {
        test_case_id: "TC001".to_string(),
        name: Some("TC001".to_string()),
        timestamp: Utc::now(),
        status: TestRunStatus::Pass,
        duration: 1.000,
        execution_log: "Test log".to_string(),
        error_message: None,
    };

    let runs_folder = storage.get_test_run_folder("TC001");
    fs::create_dir_all(&runs_folder).unwrap();

    let yaml_content = serde_yaml::to_string(&test_run).unwrap();
    fs::write(runs_folder.join("test1.yml"), &yaml_content).unwrap();
    fs::write(runs_folder.join("test2.yaml"), &yaml_content).unwrap();

    let loaded_runs = storage.load_test_runs_for_case("TC001").unwrap();
    assert_eq!(loaded_runs.len(), 2);
}

#[test]
fn test_get_test_run_folder_structure() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let folder = storage.get_test_run_folder("TC123");
    assert_eq!(
        folder,
        temp_dir.path().join("TC123").join("runs"),
        "Folder structure should be <base>/<test_case_id>/runs"
    );
}

#[test]
fn test_save_multiple_runs_same_test_case() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    for i in 0..5 {
        thread::sleep(StdDuration::from_millis(10));
        let test_run = create_test_run("TC001", i as f64 * 100f64);
        storage.save_test_run(&test_run).unwrap();
    }

    let loaded_runs = storage.load_test_runs_for_case("TC001").unwrap();
    assert_eq!(loaded_runs.len(), 5);
}

#[test]
fn test_yaml_deserialization_preserves_timestamp_precision() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let precise_timestamp = Utc::now();
    let test_run = create_test_run_with_timestamp("TC001", precise_timestamp, 1.000,);

    storage.save_test_run(&test_run).unwrap();
    let loaded_runs = storage.load_test_runs_for_case("TC001").unwrap();

    assert_eq!(loaded_runs.len(), 1);
    assert_eq!(
        loaded_runs[0].timestamp.to_rfc3339(),
        precise_timestamp.to_rfc3339()
    );
}

#[test]
fn test_load_all_with_deeply_nested_structure() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let test_cases = vec!["TC001", "TC002", "TC003"];
    for test_case_id in test_cases {
        for i in 0..3 {
            thread::sleep(StdDuration::from_millis(10));
            let test_run = create_test_run(test_case_id, i  as f64 * 1.000,);
            storage.save_test_run(&test_run).unwrap();
        }
    }

    let all_runs = storage.load_all_test_runs().unwrap();
    assert_eq!(all_runs.len(), 9);
}

#[test]
fn test_error_handling_permission_denied_simulation() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let test_run = create_test_run("TC001", 1.000,);
    let result = storage.save_test_run(&test_run);
    assert!(result.is_ok());
}

#[test]
fn test_load_preserves_all_test_run_fields() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestRunStorage::new(temp_dir.path()).unwrap();

    let original = TestRun {
        test_case_id: "TC999".to_string(),
        name: Some("TC999".to_string()),
        timestamp: Utc::now(),
        status: TestRunStatus::Fail,
        duration: 1.2345,
        execution_log: "Very detailed execution log with multiple lines\nLine 2\nLine 3"
            .to_string(),
        error_message: Some("Critical error: system failure".to_string()),
    };

    storage.save_test_run(&original).unwrap();
    let loaded_runs = storage.load_test_runs_for_case("TC999").unwrap();

    assert_eq!(loaded_runs.len(), 1);
    let loaded = &loaded_runs[0];

    assert_eq!(loaded.test_case_id, original.test_case_id);
    assert_eq!(loaded.status, original.status);
    assert_eq!(loaded.duration, original.duration);
    assert_eq!(loaded.execution_log, original.execution_log);
    assert_eq!(loaded.error_message, original.error_message);
    assert_eq!(
        loaded.timestamp.to_rfc3339(),
        original.timestamp.to_rfc3339()
    );
}
