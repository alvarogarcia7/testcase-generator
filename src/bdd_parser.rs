use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Trait for matching and extracting parameters from BDD-style statements
pub trait BddStepMatcher {
    /// Checks if a statement matches this step definition
    fn matches(&self, statement: &str) -> bool;
    
    /// Extracts named parameters from a matching statement
    fn extract_parameters(&self, statement: &str) -> Option<HashMap<String, String>>;
}

#[derive(Debug, Deserialize)]
struct TomlStepDefinition {
    #[allow(dead_code)]
    name: String,
    pattern: String,
    command_template: String,
    #[allow(dead_code)]
    description: String,
    parameters: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct TomlStepDefinitions {
    step: Vec<TomlStepDefinition>,
}

/// A BDD step definition that matches natural language patterns to shell commands
#[derive(Debug, Clone)]
pub struct BddStepDefinition {
    pub pattern: Regex,
    pub command_template: String,
}

impl BddStepDefinition {
    /// Creates a new BDD step definition with the given regex pattern and command template
    ///
    /// # Arguments
    /// * `pattern` - A regex pattern string with named capture groups
    /// * `command_template` - A template string with {parameter} placeholders
    ///
    /// # Returns
    /// A Result containing the BddStepDefinition or a regex error
    pub fn new(pattern: &str, command_template: String) -> Result<Self, regex::Error> {
        let pattern = Regex::new(pattern)?;
        Ok(Self {
            pattern,
            command_template,
        })
    }
}

impl BddStepMatcher for BddStepDefinition {
    fn matches(&self, statement: &str) -> bool {
        self.pattern.is_match(statement)
    }

    fn extract_parameters(&self, statement: &str) -> Option<HashMap<String, String>> {
        self.pattern.captures(statement).map(|caps| {
            let mut params = HashMap::new();
            for name in self.pattern.capture_names().flatten() {
                if let Some(value) = caps.name(name) {
                    params.insert(name.to_string(), value.as_str().to_string());
                }
            }
            params
        })
    }
}

/// Parses a BDD statement and generates a shell command
///
/// # Arguments
/// * `step_def` - The BDD step definition to match against
/// * `statement` - The natural language statement to parse
///
/// # Returns
/// Some(String) containing the generated command if the statement matches, None otherwise
///
/// # Security
/// Parameters extracted from the statement are inserted directly into the command template.
/// The caller is responsible for ensuring the generated command is properly escaped when
/// used in a shell context.
pub fn parse_bdd_statement(step_def: &BddStepDefinition, statement: &str) -> Option<String> {
    if !step_def.matches(statement) {
        return None;
    }

    let params = step_def.extract_parameters(statement)?;

    let mut result = step_def.command_template.clone();
    for (key, value) in params.iter() {
        let placeholder = format!("{{{}}}", key);
        result = result.replace(&placeholder, value);
    }

    Some(result)
}

/// Registry of BDD step definitions loaded from a TOML configuration file
#[derive(Debug)]
pub struct BddStepRegistry {
    step_definitions: Vec<BddStepDefinition>,
}

impl BddStepRegistry {
    /// Creates a new empty BDD step registry
    pub fn new() -> Self {
        Self {
            step_definitions: Vec::new(),
        }
    }

    /// Loads BDD step definitions from a TOML file
    ///
    /// # Arguments
    /// * `path` - Path to the TOML file containing step definitions
    ///
    /// # Returns
    /// A Result containing the registry or an error if loading fails
    pub fn load_from_toml<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let toml_defs: TomlStepDefinitions = toml::from_str(&content)?;

        let mut step_definitions = Vec::new();
        for toml_step in toml_defs.step {
            let named_pattern =
                Self::convert_to_named_groups(&toml_step.pattern, &toml_step.parameters)?;
            let step_def = BddStepDefinition::new(&named_pattern, toml_step.command_template)?;
            step_definitions.push(step_def);
        }

        Ok(Self { step_definitions })
    }

    fn convert_to_named_groups(
        pattern: &str,
        param_names: &[String],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut result = pattern.to_string();
        let capture_pattern = r"\([^)]+\)";
        let re = Regex::new(capture_pattern)?;

        // Find all capture groups first
        let mut captures: Vec<(usize, usize, String)> = Vec::new();
        for mat in re.find_iter(pattern) {
            let captured = mat.as_str();
            if !captured.starts_with("(?P<") {
                let inner = captured[1..captured.len() - 1].to_string();
                captures.push((mat.start(), mat.end(), inner));
            }
        }

        // Replace from end to start to maintain positions
        for (i, (start, end, inner)) in captures.iter().enumerate().rev() {
            if i < param_names.len() {
                let param_name = &param_names[i];
                let named = format!("(?P<{}>{})", param_name, inner);
                result.replace_range(*start..*end, &named);
            }
        }

        Ok(result)
    }

    /// Attempts to parse a statement as a BDD pattern
    ///
    /// # Arguments
    /// * `statement` - The natural language statement to parse
    ///
    /// # Returns
    /// Some(String) containing the generated command if any pattern matches, None otherwise
    pub fn try_parse_as_bdd(&self, statement: &str) -> Option<String> {
        for step_def in &self.step_definitions {
            if let Some(command) = parse_bdd_statement(step_def, statement) {
                return Some(command);
            }
        }
        None
    }
}

impl Default for BddStepRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bdd_step_definition_creation() {
        let step_def = BddStepDefinition::new(
            r"^I login as (?P<username>\w+)$",
            "login {username}".to_string(),
        );
        assert!(step_def.is_ok());
    }

    #[test]
    fn test_bdd_step_matcher_matches() {
        let step_def = BddStepDefinition::new(
            r"^I login as (?P<username>\w+)$",
            "login {username}".to_string(),
        )
        .unwrap();

        assert!(step_def.matches("I login as admin"));
        assert!(!step_def.matches("I logout"));
    }

    #[test]
    fn test_extract_parameters() {
        let step_def = BddStepDefinition::new(
            r"^I login as (?P<username>\w+)$",
            "login {username}".to_string(),
        )
        .unwrap();

        let params = step_def.extract_parameters("I login as admin");
        assert!(params.is_some());
        let params = params.unwrap();
        assert_eq!(params.get("username"), Some(&"admin".to_string()));
    }

    #[test]
    fn test_parse_bdd_statement() {
        let step_def = BddStepDefinition::new(
            r"^I login as (?P<username>\w+)$",
            "login {username}".to_string(),
        )
        .unwrap();

        let result = parse_bdd_statement(&step_def, "I login as admin");
        assert_eq!(result, Some("login admin".to_string()));
    }

    #[test]
    fn test_parse_bdd_statement_multiple_params() {
        let step_def = BddStepDefinition::new(
            r"^I send (?P<amount>\d+) to (?P<recipient>\w+)$",
            "transfer {amount} {recipient}".to_string(),
        )
        .unwrap();

        let result = parse_bdd_statement(&step_def, "I send 100 to alice");
        assert_eq!(result, Some("transfer 100 alice".to_string()));
    }

    #[test]
    fn test_parse_bdd_statement_no_match() {
        let step_def = BddStepDefinition::new(
            r"^I login as (?P<username>\w+)$",
            "login {username}".to_string(),
        )
        .unwrap();

        let result = parse_bdd_statement(&step_def, "I logout");
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_bdd_statement_complex_pattern() {
        let step_def = BddStepDefinition::new(
            r#"^I set (?P<key>\w+) to "(?P<value>[^"]+)"$"#,
            "set {key}={value}".to_string(),
        )
        .unwrap();

        let result = parse_bdd_statement(&step_def, r#"I set name to "John Doe""#);
        assert_eq!(result, Some("set name=John Doe".to_string()));
    }

    #[test]
    fn test_bdd_step_registry_new() {
        let registry = BddStepRegistry::new();
        assert_eq!(registry.step_definitions.len(), 0);
    }

    #[test]
    fn test_bdd_step_registry_try_parse_empty() {
        let registry = BddStepRegistry::new();
        let result = registry.try_parse_as_bdd("create directory \"/tmp/test\"");
        assert_eq!(result, None);
    }

    #[test]
    fn test_convert_to_named_groups_single_param() {
        let pattern = r#"^create directory "([^"]+)"$"#;
        let params = vec!["path".to_string()];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        assert_eq!(result, r#"^create directory "(?P<path>[^"]+)"$"#);
    }

    #[test]
    fn test_convert_to_named_groups_multiple_params() {
        let pattern = r#"^set environment variable "([^"]+)" to "([^"]+)"$"#;
        let params = vec!["name".to_string(), "value".to_string()];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        assert_eq!(
            result,
            r#"^set environment variable "(?P<name>[^"]+)" to "(?P<value>[^"]+)"$"#
        );
    }

    #[test]
    fn test_convert_to_named_groups_numeric_pattern() {
        let pattern = r"^wait for (\d+) seconds?$";
        let params = vec!["seconds".to_string()];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        assert_eq!(result, r"^wait for (?P<seconds>\d+) seconds?$");
    }

    #[test]
    fn test_bdd_step_registry_load_from_toml() {
        let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml");
        assert!(registry.is_ok());
        let registry = registry.unwrap();
        assert!(!registry.step_definitions.is_empty());
    }

    #[test]
    fn test_bdd_step_registry_try_parse_create_directory() {
        let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();
        let result = registry.try_parse_as_bdd("create directory \"/tmp/test\"");
        assert_eq!(result, Some("mkdir -p \"/tmp/test\"".to_string()));
    }

    #[test]
    fn test_bdd_step_registry_try_parse_wait_for_seconds() {
        let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();
        let result = registry.try_parse_as_bdd("wait for 5 seconds");
        assert_eq!(result, Some("sleep 5".to_string()));
    }

    #[test]
    fn test_bdd_step_registry_try_parse_set_env_var() {
        let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();
        let result =
            registry.try_parse_as_bdd("set environment variable \"PATH\" to \"/usr/local/bin\"");
        assert_eq!(result, Some("export PATH=/usr/local/bin".to_string()));
    }

    #[test]
    fn test_bdd_step_registry_try_parse_no_match() {
        let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();
        let result = registry.try_parse_as_bdd("this is not a valid BDD statement");
        assert_eq!(result, None);
    }

    #[test]
    fn test_bdd_step_registry_try_parse_ping_device() {
        let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();
        let result = registry.try_parse_as_bdd("ping device \"192.168.1.1\" with 3 retries");
        assert_eq!(result, Some("ping -c 3 \"192.168.1.1\"".to_string()));
    }

    #[test]
    fn test_bdd_step_registry_try_parse_check_file_exists() {
        let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();
        let result = registry.try_parse_as_bdd("file \"/etc/hosts\" should exist");
        assert_eq!(result, Some("test -f \"/etc/hosts\"".to_string()));
    }

    #[test]
    fn test_pattern_matching_exact_match() {
        let step_def =
            BddStepDefinition::new(r"^I click the button$", "click_button".to_string()).unwrap();

        assert!(step_def.matches("I click the button"));
        assert!(!step_def.matches("I click the button now"));
        assert!(!step_def.matches("First I click the button"));
    }

    #[test]
    fn test_pattern_matching_case_sensitive() {
        let step_def =
            BddStepDefinition::new(r"^I login as (?P<user>\w+)$", "login {user}".to_string())
                .unwrap();

        assert!(step_def.matches("I login as admin"));
        assert!(!step_def.matches("i login as admin"));
        assert!(!step_def.matches("I LOGIN as admin"));
    }

    #[test]
    fn test_pattern_matching_with_optional_groups() {
        let step_def = BddStepDefinition::new(
            r"^wait for (?P<seconds>\d+) seconds?$",
            "sleep {seconds}".to_string(),
        )
        .unwrap();

        assert!(step_def.matches("wait for 1 second"));
        assert!(step_def.matches("wait for 5 seconds"));
        assert!(!step_def.matches("wait for 0 seconds"));
    }

    #[test]
    fn test_parameter_extraction_single_param() {
        let step_def = BddStepDefinition::new(
            r"^I wait (?P<duration>\d+) milliseconds$",
            "sleep {duration}".to_string(),
        )
        .unwrap();

        let params = step_def.extract_parameters("I wait 500 milliseconds");
        assert!(params.is_some());
        let params = params.unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params.get("duration"), Some(&"500".to_string()));
    }

    #[test]
    fn test_parameter_extraction_multiple_params() {
        let step_def = BddStepDefinition::new(
            r#"^I copy "(?P<src>[^"]+)" to "(?P<dest>[^"]+)"$"#,
            "cp {src} {dest}".to_string(),
        )
        .unwrap();

        let params = step_def.extract_parameters(r#"I copy "/tmp/source.txt" to "/tmp/dest.txt""#);
        assert!(params.is_some());
        let params = params.unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params.get("src"), Some(&"/tmp/source.txt".to_string()));
        assert_eq!(params.get("dest"), Some(&"/tmp/dest.txt".to_string()));
    }

    #[test]
    fn test_parameter_extraction_no_match() {
        let step_def =
            BddStepDefinition::new(r"^I login as (?P<user>\w+)$", "login {user}".to_string())
                .unwrap();

        let params = step_def.extract_parameters("I logout");
        assert!(params.is_none());
    }

    #[test]
    fn test_parameter_extraction_special_chars() {
        let step_def = BddStepDefinition::new(
            r#"^file "(?P<path>[^"]+)" should contain "(?P<text>[^"]+)"$"#,
            "grep -q \"{text}\" {path}".to_string(),
        )
        .unwrap();

        let params = step_def.extract_parameters(
            r#"file "/var/log/app.log" should contain "ERROR: Failed to connect""#,
        );
        assert!(params.is_some());
        let params = params.unwrap();
        assert_eq!(params.get("path"), Some(&"/var/log/app.log".to_string()));
        assert_eq!(
            params.get("text"),
            Some(&"ERROR: Failed to connect".to_string())
        );
    }

    #[test]
    fn test_file_creation_step_basic() {
        let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();
        let result = registry.try_parse_as_bdd(r#"create file "/tmp/test.txt" with content:"#);
        assert!(result.is_some());
        let cmd = result.unwrap();
        assert!(cmd.contains("echo"));
        assert!(cmd.contains("/tmp/test.txt"));
    }

    #[test]
    fn test_file_creation_step_with_path() {
        let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();
        let result =
            registry.try_parse_as_bdd(r#"create file "/var/www/html/index.html" with content:"#);
        assert!(result.is_some());
        let cmd = result.unwrap();
        assert!(cmd.contains("/var/www/html/index.html"));
    }

    #[test]
    fn test_ping_step_with_retries_basic() {
        let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();
        let result = registry.try_parse_as_bdd(r#"ping device "192.168.1.1" with 3 retries"#);
        assert_eq!(result, Some("ping -c 3 \"192.168.1.1\"".to_string()));
    }

    #[test]
    fn test_ping_step_with_retries_single() {
        let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();
        let result = registry.try_parse_as_bdd(r#"ping device "10.0.0.1" with 1 retries"#);
        assert_eq!(result, Some("ping -c 1 \"10.0.0.1\"".to_string()));
    }

    #[test]
    fn test_ping_step_with_retries_large_number() {
        let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();
        let result = registry.try_parse_as_bdd(r#"ping device "8.8.8.8" with 100 retries"#);
        assert_eq!(result, Some("ping -c 100 \"8.8.8.8\"".to_string()));
    }

    #[test]
    fn test_ping_step_with_retries_hostname() {
        let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();
        let result = registry.try_parse_as_bdd(r#"ping device "localhost" with 5 retries"#);
        assert_eq!(result, Some("ping -c 5 \"localhost\"".to_string()));
    }

    #[test]
    fn test_template_substitution_single_placeholder() {
        let step_def = BddStepDefinition::new(
            r"^delete file (?P<filename>\S+)$",
            "rm {filename}".to_string(),
        )
        .unwrap();

        let result = parse_bdd_statement(&step_def, "delete file /tmp/test.txt");
        assert_eq!(result, Some("rm /tmp/test.txt".to_string()));
    }

    #[test]
    fn test_template_substitution_multiple_placeholders() {
        let step_def = BddStepDefinition::new(
            r"^move (?P<src>\S+) to (?P<dest>\S+)$",
            "mv {src} {dest}".to_string(),
        )
        .unwrap();

        let result = parse_bdd_statement(&step_def, "move /tmp/old.txt /tmp/new.txt");
        assert_eq!(result, Some("mv /tmp/old.txt /tmp/new.txt".to_string()));
    }

    #[test]
    fn test_template_substitution_repeated_placeholder() {
        let step_def = BddStepDefinition::new(
            r"^backup (?P<file>\S+)$",
            "cp {file} {file}.bak".to_string(),
        )
        .unwrap();

        let result = parse_bdd_statement(&step_def, "backup /etc/config");
        assert_eq!(result, Some("cp /etc/config /etc/config.bak".to_string()));
    }

    #[test]
    fn test_template_substitution_no_placeholders() {
        let step_def =
            BddStepDefinition::new(r"^reboot system$", "sudo reboot".to_string()).unwrap();

        let result = parse_bdd_statement(&step_def, "reboot system");
        assert_eq!(result, Some("sudo reboot".to_string()));
    }

    #[test]
    fn test_template_substitution_empty_value() {
        let step_def = BddStepDefinition::new(
            r"^set variable (?P<name>\w+) to (?P<value>.*)$",
            "export {name}={value}".to_string(),
        )
        .unwrap();

        let result = parse_bdd_statement(&step_def, "set variable DEBUG to ");
        assert_eq!(result, Some("export DEBUG=".to_string()));
    }

    #[test]
    fn test_template_substitution_special_characters() {
        let step_def = BddStepDefinition::new(
            r#"^echo message "(?P<msg>[^"]+)"$"#,
            "echo \"{msg}\"".to_string(),
        )
        .unwrap();

        let result = parse_bdd_statement(&step_def, r#"echo message "Hello, World! @#$%""#);
        assert_eq!(result, Some(r#"echo "Hello, World! @#$%""#.to_string()));
    }

    #[test]
    fn test_template_substitution_missing_placeholder() {
        let step_def = BddStepDefinition::new(
            r"^run command (?P<cmd>\w+)$",
            "{cmd} --verbose --output {outfile}".to_string(),
        )
        .unwrap();

        let result = parse_bdd_statement(&step_def, "run command test");
        assert_eq!(
            result,
            Some("test --verbose --output {outfile}".to_string())
        );
    }

    #[test]
    fn test_template_substitution_numeric_values() {
        let step_def = BddStepDefinition::new(
            r"^retry (?P<count>\d+) times with (?P<delay>\d+) seconds delay$",
            "for i in $(seq 1 {count}); do sleep {delay}; done".to_string(),
        )
        .unwrap();

        let result = parse_bdd_statement(&step_def, "retry 3 times with 5 seconds delay");
        assert_eq!(
            result,
            Some("for i in $(seq 1 3); do sleep 5; done".to_string())
        );
    }

    #[test]
    fn test_template_substitution_path_with_spaces() {
        let step_def = BddStepDefinition::new(
            r#"^remove directory "(?P<path>[^"]+)"$"#,
            "rm -rf \"{path}\"".to_string(),
        )
        .unwrap();

        let result = parse_bdd_statement(&step_def, r#"remove directory "/tmp/my folder""#);
        assert_eq!(result, Some("rm -rf \"/tmp/my folder\"".to_string()));
    }

    #[test]
    fn test_edge_case_empty_string_parameter() {
        let step_def = BddStepDefinition::new(
            r#"^append "(?P<text>.*)" to log$"#,
            "echo \"{text}\" >> /var/log/app.log".to_string(),
        )
        .unwrap();

        let result = parse_bdd_statement(&step_def, r#"append "" to log"#);
        assert_eq!(result, Some(r#"echo "" >> /var/log/app.log"#.to_string()));
    }

    #[test]
    fn test_edge_case_placeholder_in_quotes() {
        let step_def = BddStepDefinition::new(
            r"^greet (?P<name>\w+)$",
            "echo 'Hello, {name}!'".to_string(),
        )
        .unwrap();

        let result = parse_bdd_statement(&step_def, "greet Alice");
        assert_eq!(result, Some("echo 'Hello, Alice!'".to_string()));
    }

    #[test]
    fn test_edge_case_multiple_same_placeholders() {
        let step_def = BddStepDefinition::new(
            r"^mirror (?P<value>\w+)$",
            "{value} equals {value}".to_string(),
        )
        .unwrap();

        let result = parse_bdd_statement(&step_def, "mirror test");
        assert_eq!(result, Some("test equals test".to_string()));
    }

    #[test]
    fn test_convert_to_named_groups_no_capture_groups() {
        // Pattern with no capture groups at all
        let pattern = r"^simple pattern with no captures$";
        let params = vec![];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        assert_eq!(result, r"^simple pattern with no captures$");
    }

    #[test]
    fn test_convert_to_named_groups_already_named() {
        // Pattern with already named groups should not be modified
        let pattern = r#"^already named (?P<param1>\w+) and (?P<param2>\d+)$"#;
        let params = vec!["param1".to_string(), "param2".to_string()];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        assert_eq!(result, r#"^already named (?P<param1>\w+) and (?P<param2>\d+)$"#);
    }

    #[test]
    fn test_convert_to_named_groups_mixed_named_and_unnamed() {
        // Pattern with mix of named and unnamed groups
        let pattern = r#"^mixed (?P<named>\w+) and (\d+)$"#;
        let params = vec!["first".to_string(), "second".to_string()];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        // The first unnamed group (index 0) should get the first param name
        assert_eq!(result, r#"^mixed (?P<named>\w+) and (?P<first>\d+)$"#);
    }

    #[test]
    fn test_convert_to_named_groups_more_params_than_groups() {
        // More parameter names than capture groups - extra params ignored
        let pattern = r"^single (\w+) group$";
        let params = vec!["param1".to_string(), "param2".to_string(), "param3".to_string()];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        assert_eq!(result, r"^single (?P<param1>\w+) group$");
    }

    #[test]
    fn test_convert_to_named_groups_fewer_params_than_groups() {
        // Fewer parameter names than capture groups - extra groups remain unnamed
        let pattern = r"^three (\w+) different (\d+) groups (\S+)$";
        let params = vec!["first".to_string()];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        // Only the first group should be named
        assert_eq!(result, r"^three (?P<first>\w+) different (\d+) groups (\S+)$");
    }

    #[test]
    fn test_convert_to_named_groups_special_regex_chars() {
        // Pattern with special regex characters like +, *, ?, [], etc.
        let pattern = r"^match (\d+\.\d+) or ([a-zA-Z]+\*?) with (\w+\+?)$";
        let params = vec!["decimal".to_string(), "word".to_string(), "plus".to_string()];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        assert_eq!(result, r"^match (?P<decimal>\d+\.\d+) or (?P<word>[a-zA-Z]+\*?) with (?P<plus>\w+\+?)$");
    }

    #[test]
    fn test_convert_to_named_groups_nested_parentheses_in_character_class() {
        // Pattern with parentheses inside character class - should not be treated as capture group
        let pattern = r"^test (\w+) with [()]+$";
        let params = vec!["param".to_string()];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        assert_eq!(result, r"^test (?P<param>\w+) with [()]+$");
    }

    #[test]
    fn test_convert_to_named_groups_escaped_parentheses() {
        // Pattern with escaped parentheses - the current implementation has limitations
        // The simple regex pattern doesn't distinguish between escaped and unescaped parens
        let pattern = r"^literal \(parens\) and (\w+)$";
        let params = vec!["param".to_string()];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        // Current behavior: converts first matching group including escaped parens
        assert_eq!(result, r"^literal \(?P<param>parens\) and (\w+)$");
    }

    #[test]
    fn test_convert_to_named_groups_non_capturing_groups() {
        // Pattern with non-capturing groups (?:...) mixed with capturing groups
        // The current implementation has limitations with special group syntax
        let pattern = r"^match (?:prefix|suffix) (\w+)$";
        let params = vec!["word".to_string()];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        // Current behavior: the simple regex matches any (content) pattern
        assert_eq!(result, r"^match (?P<word>?:prefix|suffix) (\w+)$");
    }

    #[test]
    fn test_convert_to_named_groups_complex_nested_pattern() {
        // Complex pattern with nested structures
        let pattern = r"^command with (\S+) and optional (\d+)?$";
        let params = vec!["arg".to_string(), "count".to_string()];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        assert_eq!(result, r"^command with (?P<arg>\S+) and optional (?P<count>\d+)?$");
    }

    #[test]
    fn test_convert_to_named_groups_character_classes_with_special_chars() {
        // Pattern with complex character classes
        let pattern = r#"^path "([^"]+)" and value ([^\s]+)$"#;
        let params = vec!["path".to_string(), "value".to_string()];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        assert_eq!(result, r#"^path "(?P<path>[^"]+)" and value (?P<value>[^\s]+)$"#);
    }

    #[test]
    fn test_convert_to_named_groups_alternation_in_capture() {
        // Pattern with alternation inside capture group
        let pattern = r"^choose (yes|no|maybe)$";
        let params = vec!["choice".to_string()];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        assert_eq!(result, r"^choose (?P<choice>yes|no|maybe)$");
    }

    #[test]
    fn test_convert_to_named_groups_empty_params_list() {
        // Pattern with capture groups but no parameter names
        let pattern = r"^capture (\w+) and (\d+)$";
        let params = vec![];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        // No groups should be named
        assert_eq!(result, r"^capture (\w+) and (\d+)$");
    }

    #[test]
    fn test_convert_to_named_groups_quantifiers_outside_groups() {
        // Pattern with quantifiers outside capture groups
        let pattern = r"^repeat (\w)+ times (\d+)$";
        let params = vec!["letter".to_string(), "count".to_string()];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        assert_eq!(result, r"^repeat (?P<letter>\w)+ times (?P<count>\d+)$");
    }

    #[test]
    fn test_convert_to_named_groups_lookahead_lookbehind() {
        // Pattern with lookahead/lookbehind assertions
        let pattern = r"^find (\w+)(?= is found)$";
        let params = vec!["word".to_string()];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        // Lookahead is not a capturing group and should not be modified
        assert_eq!(result, r"^find (?P<word>\w+)(?= is found)$");
    }

    #[test]
    fn test_convert_to_named_groups_backreferences() {
        // Pattern with backreferences
        let pattern = r"^match (\w+) and repeat \1$";
        let params = vec!["word".to_string()];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        // After conversion, backreference may break but conversion should still work
        assert_eq!(result, r"^match (?P<word>\w+) and repeat \1$");
    }

    #[test]
    fn test_convert_to_named_groups_unicode_patterns() {
        // Pattern with unicode character classes
        let pattern = r"^unicode (\p{L}+) text$";
        let params = vec!["text".to_string()];
        let result = BddStepRegistry::convert_to_named_groups(pattern, &params).unwrap();
        assert_eq!(result, r"^unicode (?P<text>\p{L}+) text$");
    }
}
