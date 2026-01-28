use testcase_manager::BddStepRegistry;

#[test]
fn test_all_23_step_definitions_loaded() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml")
        .expect("Failed to load BDD step definitions");

    assert!(registry
        .try_parse_as_bdd("create file \"/tmp/test.txt\" with content:")
        .is_some());
}

#[test]
fn test_01_create_file_with_content_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"create file "/tmp/test.txt" with content:"#);
    assert_eq!(result, Some("touch \"/tmp/test.txt\"".to_string()));

    let result =
        registry.try_parse_as_bdd(r#"create file "/var/www/html/index.html" with content:"#);
    assert_eq!(
        result,
        Some("touch \"/var/www/html/index.html\"".to_string())
    );

    let result = registry.try_parse_as_bdd(r#"create file "output.log" with content:"#);
    assert_eq!(result, Some("touch \"output.log\"".to_string()));
}

#[test]
fn test_01_create_file_with_content_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(
        registry.try_parse_as_bdd("create file without content:"),
        None
    );
    assert_eq!(registry.try_parse_as_bdd("create file"), None);
    assert_eq!(
        registry.try_parse_as_bdd(r#"create file "/tmp/test.txt""#),
        None
    );
}

#[test]
fn test_02_ping_device_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"ping device "192.168.1.1" with 3 retries"#);
    assert_eq!(result, Some("ping -c 3 \"192.168.1.1\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"ping device "8.8.8.8" with 10 retries"#);
    assert_eq!(result, Some("ping -c 10 \"8.8.8.8\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"ping device "localhost" with 1 retries"#);
    assert_eq!(result, Some("ping -c 1 \"localhost\"".to_string()));
}

#[test]
fn test_02_ping_device_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(
        registry.try_parse_as_bdd(r#"ping device "192.168.1.1""#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"ping device with 3 retries"#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"ping "192.168.1.1" with abc retries"#),
        None
    );
}

#[test]
fn test_03_check_file_exists_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"file "/etc/hosts" should exist"#);
    assert_eq!(result, Some("test -f \"/etc/hosts\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"file "/tmp/test.log" should exist"#);
    assert_eq!(result, Some("test -f \"/tmp/test.log\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"file "config.yaml" should exist"#);
    assert_eq!(result, Some("test -f \"config.yaml\"".to_string()));
}

#[test]
fn test_03_check_file_exists_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(registry.try_parse_as_bdd(r#"file should exist"#), None);
    assert_eq!(
        registry.try_parse_as_bdd(r#"file "/etc/hosts" exists"#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"/etc/hosts should exist"#),
        None
    );
}

#[test]
fn test_04_create_directory_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"create directory "/tmp/test""#);
    assert_eq!(result, Some("mkdir -p \"/tmp/test\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"create directory "/var/log/app""#);
    assert_eq!(result, Some("mkdir -p \"/var/log/app\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"create directory "output""#);
    assert_eq!(result, Some("mkdir -p \"output\"".to_string()));
}

#[test]
fn test_04_create_directory_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(registry.try_parse_as_bdd("create directory"), None);
    assert_eq!(registry.try_parse_as_bdd("create /tmp/test"), None);
    assert_eq!(registry.try_parse_as_bdd("mkdir /tmp/test"), None);
}

#[test]
fn test_05_remove_directory_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"remove directory "/tmp/test""#);
    assert_eq!(result, Some("rm -rf \"/tmp/test\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"remove directory "/var/tmp/old_data""#);
    assert_eq!(result, Some("rm -rf \"/var/tmp/old_data\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"remove directory "build""#);
    assert_eq!(result, Some("rm -rf \"build\"".to_string()));
}

#[test]
fn test_05_remove_directory_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(registry.try_parse_as_bdd("remove directory"), None);
    assert_eq!(
        registry.try_parse_as_bdd("delete directory \"/tmp/test\""),
        None
    );
    assert_eq!(registry.try_parse_as_bdd("rm -rf /tmp/test"), None);
}

#[test]
fn test_06_list_directory_contents_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"list contents of directory "/tmp""#);
    assert_eq!(result, Some("ls -la \"/tmp\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"list contents of directory "/var/log""#);
    assert_eq!(result, Some("ls -la \"/var/log\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"list contents of directory ".""#);
    assert_eq!(result, Some("ls -la \".\"".to_string()));
}

#[test]
fn test_06_list_directory_contents_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(registry.try_parse_as_bdd("list directory \"/tmp\""), None);
    assert_eq!(registry.try_parse_as_bdd("list contents of /tmp"), None);
    assert_eq!(registry.try_parse_as_bdd("ls -la /tmp"), None);
}

#[test]
fn test_07_set_environment_variable_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result =
        registry.try_parse_as_bdd(r#"set environment variable "PATH" to "/usr/local/bin""#);
    assert_eq!(result, Some("export PATH=/usr/local/bin".to_string()));

    let result = registry.try_parse_as_bdd(r#"set environment variable "DEBUG" to "true""#);
    assert_eq!(result, Some("export DEBUG=true".to_string()));

    let result = registry.try_parse_as_bdd(r#"set environment variable "APP_VERSION" to "1.0.0""#);
    assert_eq!(result, Some("export APP_VERSION=1.0.0".to_string()));
}

#[test]
fn test_07_set_environment_variable_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(
        registry.try_parse_as_bdd("set environment variable \"PATH\""),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd("set PATH to \"/usr/local/bin\""),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd("export PATH=/usr/local/bin"),
        None
    );
}

#[test]
fn test_08_unset_environment_variable_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"unset environment variable "DEBUG""#);
    assert_eq!(result, Some("unset DEBUG".to_string()));

    let result = registry.try_parse_as_bdd(r#"unset environment variable "TEMP_VAR""#);
    assert_eq!(result, Some("unset TEMP_VAR".to_string()));

    let result = registry.try_parse_as_bdd(r#"unset environment variable "PATH""#);
    assert_eq!(result, Some("unset PATH".to_string()));
}

#[test]
fn test_08_unset_environment_variable_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(
        registry.try_parse_as_bdd("unset environment variable"),
        None
    );
    assert_eq!(registry.try_parse_as_bdd("unset DEBUG"), None);
    assert_eq!(
        registry.try_parse_as_bdd("remove environment variable \"DEBUG\""),
        None
    );
}

#[test]
fn test_09_check_process_running_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"process "nginx" should be running"#);
    assert_eq!(result, Some("pgrep -f \"nginx\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"process "postgres" should be running"#);
    assert_eq!(result, Some("pgrep -f \"postgres\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"process "java -jar app.jar" should be running"#);
    assert_eq!(result, Some("pgrep -f \"java -jar app.jar\"".to_string()));
}

#[test]
fn test_09_check_process_running_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(registry.try_parse_as_bdd("process should be running"), None);
    assert_eq!(registry.try_parse_as_bdd("nginx should be running"), None);
    assert_eq!(
        registry.try_parse_as_bdd(r#"process "nginx" is running"#),
        None
    );
}

#[test]
fn test_10_kill_process_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"kill process "nginx""#);
    assert_eq!(result, Some("pkill -f \"nginx\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"kill process "python app.py""#);
    assert_eq!(result, Some("pkill -f \"python app.py\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"kill process "java""#);
    assert_eq!(result, Some("pkill -f \"java\"".to_string()));
}

#[test]
fn test_10_kill_process_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(registry.try_parse_as_bdd("kill process"), None);
    assert_eq!(registry.try_parse_as_bdd("kill nginx"), None);
    assert_eq!(
        registry.try_parse_as_bdd(r#"terminate process "nginx""#),
        None
    );
}

#[test]
fn test_11_change_file_permissions_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"change permissions of "/tmp/test.sh" to 755"#);
    assert_eq!(result, Some("chmod 755 \"/tmp/test.sh\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"change permissions of "/etc/config" to 644"#);
    assert_eq!(result, Some("chmod 644 \"/etc/config\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"change permissions of "script.sh" to 0755"#);
    assert_eq!(result, Some("chmod 0755 \"script.sh\"".to_string()));
}

#[test]
fn test_11_change_file_permissions_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(
        registry.try_parse_as_bdd(r#"change permissions of "/tmp/test.sh""#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"chmod "/tmp/test.sh" to 755"#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"change permissions of "/tmp/test.sh" to rwxr-xr-x"#),
        None
    );
}

#[test]
fn test_12_check_file_contains_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"file "/var/log/app.log" should contain "ERROR""#);
    assert_eq!(
        result,
        Some("grep -q \"ERROR\" \"/var/log/app.log\"".to_string())
    );

    let result = registry.try_parse_as_bdd(r#"file "/etc/hosts" should contain "localhost""#);
    assert_eq!(
        result,
        Some("grep -q \"localhost\" \"/etc/hosts\"".to_string())
    );

    let result = registry.try_parse_as_bdd(r#"file "config.txt" should contain "debug=true""#);
    assert_eq!(
        result,
        Some("grep -q \"debug=true\" \"config.txt\"".to_string())
    );
}

#[test]
fn test_12_check_file_contains_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(
        registry.try_parse_as_bdd(r#"file "/var/log/app.log" should contain"#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"file should contain "ERROR""#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"/var/log/app.log should contain "ERROR""#),
        None
    );
}

#[test]
fn test_13_append_to_file_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"append "Hello World" to file "/tmp/test.txt""#);
    assert_eq!(
        result,
        Some("echo \"Hello World\" >> \"/tmp/test.txt\"".to_string())
    );

    let result = registry.try_parse_as_bdd(r#"append "log entry" to file "/var/log/app.log""#);
    assert_eq!(
        result,
        Some("echo \"log entry\" >> \"/var/log/app.log\"".to_string())
    );

    let result = registry.try_parse_as_bdd(r#"append "new line" to file "output.txt""#);
    assert_eq!(
        result,
        Some("echo \"new line\" >> \"output.txt\"".to_string())
    );
}

#[test]
fn test_13_append_to_file_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(
        registry.try_parse_as_bdd(r#"append to file "/tmp/test.txt""#),
        None
    );
    assert_eq!(registry.try_parse_as_bdd(r#"append "text""#), None);
    assert_eq!(
        registry.try_parse_as_bdd(r#"add "text" to file "/tmp/test.txt""#),
        None
    );
}

#[test]
fn test_14_replace_in_file_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"replace "old" with "new" in file "/tmp/test.txt""#);
    assert_eq!(
        result,
        Some("sed -i 's/old/new/g' \"/tmp/test.txt\"".to_string())
    );

    let result = registry.try_parse_as_bdd(r#"replace "DEBUG" with "INFO" in file "/etc/config""#);
    assert_eq!(
        result,
        Some("sed -i 's/DEBUG/INFO/g' \"/etc/config\"".to_string())
    );

    let result =
        registry.try_parse_as_bdd(r#"replace "localhost" with "127.0.0.1" in file "hosts""#);
    assert_eq!(
        result,
        Some("sed -i 's/localhost/127.0.0.1/g' \"hosts\"".to_string())
    );
}

#[test]
fn test_14_replace_in_file_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(
        registry.try_parse_as_bdd(r#"replace "old" in file "/tmp/test.txt""#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"replace "old" with "new""#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"substitute "old" with "new" in file "/tmp/test.txt""#),
        None
    );
}

#[test]
fn test_15_wait_for_seconds_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd("wait for 5 seconds");
    assert_eq!(result, Some("sleep 5".to_string()));

    let result = registry.try_parse_as_bdd("wait for 1 second");
    assert_eq!(result, Some("sleep 1".to_string()));

    let result = registry.try_parse_as_bdd("wait for 120 seconds");
    assert_eq!(result, Some("sleep 120".to_string()));
}

#[test]
fn test_15_wait_for_seconds_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(registry.try_parse_as_bdd("wait for seconds"), None);
    assert_eq!(registry.try_parse_as_bdd("wait 5 seconds"), None);
    assert_eq!(registry.try_parse_as_bdd("sleep 5"), None);
}

#[test]
fn test_16_wait_until_file_exists_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result =
        registry.try_parse_as_bdd(r#"wait until file "/tmp/ready" exists with timeout 30 seconds"#);
    assert_eq!(
        result,
        Some("timeout 30 bash -c 'while [ ! -f \"/tmp/ready\" ]; do sleep 1; done'".to_string())
    );

    let result = registry
        .try_parse_as_bdd(r#"wait until file "/var/run/app.pid" exists with timeout 10 seconds"#);
    assert_eq!(
        result,
        Some(
            "timeout 10 bash -c 'while [ ! -f \"/var/run/app.pid\" ]; do sleep 1; done'"
                .to_string()
        )
    );

    let result =
        registry.try_parse_as_bdd(r#"wait until file "output.log" exists with timeout 60 seconds"#);
    assert_eq!(
        result,
        Some("timeout 60 bash -c 'while [ ! -f \"output.log\" ]; do sleep 1; done'".to_string())
    );
}

#[test]
fn test_16_wait_until_file_exists_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(
        registry.try_parse_as_bdd(r#"wait until file "/tmp/ready" exists"#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"wait until file exists with timeout 30 seconds"#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"wait for file "/tmp/ready" with timeout 30 seconds"#),
        None
    );
}

#[test]
fn test_17_check_port_open_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"port 80 on "localhost" should be open"#);
    assert_eq!(result, Some("nc -z \"localhost\" 80".to_string()));

    let result = registry.try_parse_as_bdd(r#"port 443 on "example.com" should be open"#);
    assert_eq!(result, Some("nc -z \"example.com\" 443".to_string()));

    let result = registry.try_parse_as_bdd(r#"port 8080 on "192.168.1.1" should be open"#);
    assert_eq!(result, Some("nc -z \"192.168.1.1\" 8080".to_string()));
}

#[test]
fn test_17_check_port_open_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(registry.try_parse_as_bdd(r#"port 80 should be open"#), None);
    assert_eq!(
        registry.try_parse_as_bdd(r#"port on "localhost" should be open"#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"check port 80 on "localhost""#),
        None
    );
}

#[test]
fn test_18_http_request_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"send GET request to "http://example.com""#);
    assert_eq!(
        result,
        Some("curl -X GET \"http://example.com\"".to_string())
    );

    let result = registry.try_parse_as_bdd(r#"send POST request to "http://api.example.com/data""#);
    assert_eq!(
        result,
        Some("curl -X POST \"http://api.example.com/data\"".to_string())
    );

    let result =
        registry.try_parse_as_bdd(r#"send PUT request to "http://api.example.com/users/1""#);
    assert_eq!(
        result,
        Some("curl -X PUT \"http://api.example.com/users/1\"".to_string())
    );

    let result =
        registry.try_parse_as_bdd(r#"send DELETE request to "http://api.example.com/users/1""#);
    assert_eq!(
        result,
        Some("curl -X DELETE \"http://api.example.com/users/1\"".to_string())
    );
}

#[test]
fn test_18_http_request_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(
        registry.try_parse_as_bdd(r#"send request to "http://example.com""#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"send GET to "http://example.com""#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"send PATCH request to "http://example.com""#),
        None
    );
}

#[test]
fn test_19_create_user_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"create user "testuser" with uid 1001"#);
    assert_eq!(result, Some("useradd -u 1001 \"testuser\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"create user "admin" with uid 1000"#);
    assert_eq!(result, Some("useradd -u 1000 \"admin\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"create user "appuser" with uid 2000"#);
    assert_eq!(result, Some("useradd -u 2000 \"appuser\"".to_string()));
}

#[test]
fn test_19_create_user_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(registry.try_parse_as_bdd(r#"create user "testuser""#), None);
    assert_eq!(
        registry.try_parse_as_bdd(r#"create user with uid 1001"#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"add user "testuser" with uid 1001"#),
        None
    );
}

#[test]
fn test_20_delete_user_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"delete user "testuser""#);
    assert_eq!(result, Some("userdel \"testuser\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"delete user "olduser""#);
    assert_eq!(result, Some("userdel \"olduser\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"delete user "tempuser""#);
    assert_eq!(result, Some("userdel \"tempuser\"".to_string()));
}

#[test]
fn test_20_delete_user_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(registry.try_parse_as_bdd("delete user"), None);
    assert_eq!(registry.try_parse_as_bdd("remove user \"testuser\""), None);
    assert_eq!(registry.try_parse_as_bdd("userdel testuser"), None);
}

#[test]
fn test_21_restart_service_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"restart service "nginx""#);
    assert_eq!(result, Some("systemctl restart \"nginx\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"restart service "postgresql""#);
    assert_eq!(result, Some("systemctl restart \"postgresql\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"restart service "docker""#);
    assert_eq!(result, Some("systemctl restart \"docker\"".to_string()));
}

#[test]
fn test_21_restart_service_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(registry.try_parse_as_bdd("restart service"), None);
    assert_eq!(registry.try_parse_as_bdd("restart nginx"), None);
    assert_eq!(registry.try_parse_as_bdd(r#"start service "nginx""#), None);
}

#[test]
fn test_22_extract_archive_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result =
        registry.try_parse_as_bdd(r#"extract archive "/tmp/backup.tar.gz" to "/var/restore""#);
    assert_eq!(
        result,
        Some("tar -xzf \"/tmp/backup.tar.gz\" -C \"/var/restore\"".to_string())
    );

    let result = registry.try_parse_as_bdd(r#"extract archive "app.tar.gz" to "/opt/app""#);
    assert_eq!(
        result,
        Some("tar -xzf \"app.tar.gz\" -C \"/opt/app\"".to_string())
    );

    let result = registry.try_parse_as_bdd(r#"extract archive "/downloads/data.tar.gz" to ".""#);
    assert_eq!(
        result,
        Some("tar -xzf \"/downloads/data.tar.gz\" -C \".\"".to_string())
    );
}

#[test]
fn test_22_extract_archive_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(
        registry.try_parse_as_bdd(r#"extract archive "/tmp/backup.tar.gz""#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"extract to "/var/restore""#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"untar "/tmp/backup.tar.gz" to "/var/restore""#),
        None
    );
}

#[test]
fn test_23_create_archive_valid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry
        .try_parse_as_bdd(r#"create archive "/tmp/backup.tar.gz" from directory "/var/www""#);
    assert_eq!(
        result,
        Some("tar -czf \"/tmp/backup.tar.gz\" -C \"/var/www\" .".to_string())
    );

    let result =
        registry.try_parse_as_bdd(r#"create archive "backup.tar.gz" from directory "/home/user""#);
    assert_eq!(
        result,
        Some("tar -czf \"backup.tar.gz\" -C \"/home/user\" .".to_string())
    );

    let result = registry
        .try_parse_as_bdd(r#"create archive "/backups/data.tar.gz" from directory "/opt/data""#);
    assert_eq!(
        result,
        Some("tar -czf \"/backups/data.tar.gz\" -C \"/opt/data\" .".to_string())
    );
}

#[test]
fn test_23_create_archive_invalid() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(
        registry.try_parse_as_bdd(r#"create archive "/tmp/backup.tar.gz""#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"create archive from directory "/var/www""#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"tar backup.tar.gz from /var/www"#),
        None
    );
}

#[test]
fn test_all_steps_reject_malformed_input() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(registry.try_parse_as_bdd(""), None);
    assert_eq!(registry.try_parse_as_bdd("invalid command"), None);
    assert_eq!(
        registry.try_parse_as_bdd("random text that matches nothing"),
        None
    );
    assert_eq!(registry.try_parse_as_bdd("123456"), None);
    assert_eq!(registry.try_parse_as_bdd("!@#$%^&*()"), None);
}

#[test]
fn test_registry_count_and_load() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml")
        .expect("Failed to load BDD step definitions from TOML");

    let test_cases = vec![
        (r#"create file "/tmp/test.txt" with content:"#, true),
        (r#"ping device "192.168.1.1" with 3 retries"#, true),
        (r#"file "/etc/hosts" should exist"#, true),
        (r#"create directory "/tmp/test""#, true),
        (r#"remove directory "/tmp/test""#, true),
        (r#"list contents of directory "/tmp""#, true),
        (
            r#"set environment variable "PATH" to "/usr/local/bin""#,
            true,
        ),
        (r#"unset environment variable "DEBUG""#, true),
        (r#"process "nginx" should be running"#, true),
        (r#"kill process "nginx""#, true),
        (r#"change permissions of "/tmp/test.sh" to 755"#, true),
        (r#"file "/var/log/app.log" should contain "ERROR""#, true),
        (r#"append "Hello World" to file "/tmp/test.txt""#, true),
        (r#"replace "old" with "new" in file "/tmp/test.txt""#, true),
        ("wait for 5 seconds", true),
        (
            r#"wait until file "/tmp/ready" exists with timeout 30 seconds"#,
            true,
        ),
        (r#"port 80 on "localhost" should be open"#, true),
        (r#"send GET request to "http://example.com""#, true),
        (r#"create user "testuser" with uid 1001"#, true),
        (r#"delete user "testuser""#, true),
        (r#"restart service "nginx""#, true),
        (
            r#"extract archive "/tmp/backup.tar.gz" to "/var/restore""#,
            true,
        ),
        (
            r#"create archive "/tmp/backup.tar.gz" from directory "/var/www""#,
            true,
        ),
    ];

    let mut matched_count = 0;
    for (statement, should_match) in test_cases {
        let result = registry.try_parse_as_bdd(statement);
        if should_match {
            assert!(
                result.is_some(),
                "Expected '{}' to match a pattern",
                statement
            );
            if result.is_some() {
                matched_count += 1;
            }
        } else {
            assert!(
                result.is_none(),
                "Expected '{}' to not match any pattern",
                statement
            );
        }
    }

    assert_eq!(
        matched_count, 23,
        "All 23 step definitions should match successfully"
    );
}

#[test]
fn test_edge_cases_special_characters_in_paths() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result =
        registry.try_parse_as_bdd(r#"create file "/tmp/test file with spaces.txt" with content:"#);
    assert_eq!(
        result,
        Some("touch \"/tmp/test file with spaces.txt\"".to_string())
    );

    let result = registry.try_parse_as_bdd(r#"create directory "/tmp/test-dir_123""#);
    assert_eq!(result, Some("mkdir -p \"/tmp/test-dir_123\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"file "/tmp/test@#$.log" should exist"#);
    assert_eq!(result, Some("test -f \"/tmp/test@#$.log\"".to_string()));
}

#[test]
fn test_edge_cases_numeric_values() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd("wait for 0 seconds");
    assert_eq!(result, Some("sleep 0".to_string()));

    let result = registry.try_parse_as_bdd("wait for 999999 seconds");
    assert_eq!(result, Some("sleep 999999".to_string()));

    let result = registry.try_parse_as_bdd(r#"change permissions of "/tmp/test" to 000"#);
    assert_eq!(result, Some("chmod 000 \"/tmp/test\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"change permissions of "/tmp/test" to 7777"#);
    assert_eq!(result, Some("chmod 7777 \"/tmp/test\"".to_string()));
}

#[test]
fn test_edge_cases_empty_strings() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"append " " to file "/tmp/test.txt""#);
    assert_eq!(result, Some("echo \" \" >> \"/tmp/test.txt\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"set environment variable "EMPTY" to " ""#);
    assert_eq!(result, Some("export EMPTY= ".to_string()));
}

#[test]
fn test_case_sensitivity() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(
        registry.try_parse_as_bdd(r#"CREATE FILE "/tmp/test.txt" with content:"#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"Create File "/tmp/test.txt" with content:"#),
        None
    );
    assert_eq!(registry.try_parse_as_bdd(r#"WAIT FOR 5 SECONDS"#), None);
}

#[test]
fn test_pattern_boundaries() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    assert_eq!(
        registry.try_parse_as_bdd(r#"prefix create file "/tmp/test.txt" with content:"#),
        None
    );
    assert_eq!(
        registry.try_parse_as_bdd(r#"create file "/tmp/test.txt" with content: suffix"#),
        None
    );
    assert_eq!(registry.try_parse_as_bdd("  wait for 5 seconds"), None);
}

#[test]
fn test_unicode_in_parameters() {
    let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();

    let result = registry.try_parse_as_bdd(r#"create file "/tmp/测试.txt" with content:"#);
    assert_eq!(result, Some("touch \"/tmp/测试.txt\"".to_string()));

    let result = registry.try_parse_as_bdd(r#"append "Hello 世界" to file "/tmp/test.txt""#);
    assert_eq!(
        result,
        Some("echo \"Hello 世界\" >> \"/tmp/test.txt\"".to_string())
    );
}
