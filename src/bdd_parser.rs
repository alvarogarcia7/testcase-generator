use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub trait BddStepMatcher {
    fn matches(&self, statement: &str) -> bool;
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

#[derive(Debug, Clone)]
pub struct BddStepDefinition {
    pub pattern: Regex,
    pub command_template: String,
}

impl BddStepDefinition {
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

#[derive(Debug)]
pub struct BddStepRegistry {
    step_definitions: Vec<BddStepDefinition>,
}

impl BddStepRegistry {
    pub fn new() -> Self {
        Self {
            step_definitions: Vec::new(),
        }
    }

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
        let capture_groups = vec![r"\([^)]+\)"];

        let mut param_index = 0;
        for capture_pattern in capture_groups {
            let re = Regex::new(capture_pattern)?;
            while let Some(mat) = re.find(&result) {
                if param_index >= param_names.len() {
                    break;
                }
                let param_name = &param_names[param_index];
                let start = mat.start();
                let end = mat.end();
                let captured = &result[start..end];

                if !captured.starts_with("(?P<") {
                    let inner = &captured[1..captured.len() - 1];
                    let named = format!("(?P<{}>{})", param_name, inner);
                    result.replace_range(start..end, &named);
                    param_index += 1;
                } else {
                    break;
                }
            }
        }

        Ok(result)
    }

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
        assert_eq!(result, Some("mkdir -p /tmp/test".to_string()));
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
        assert_eq!(result, Some("ping -c 3 192.168.1.1".to_string()));
    }

    #[test]
    fn test_bdd_step_registry_try_parse_check_file_exists() {
        let registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml").unwrap();
        let result = registry.try_parse_as_bdd("file \"/etc/hosts\" should exist");
        assert_eq!(result, Some("test -f /etc/hosts".to_string()));
    }
}
