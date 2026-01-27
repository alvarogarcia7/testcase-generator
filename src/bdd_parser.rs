use regex::Regex;
use std::collections::HashMap;

pub trait BddStepMatcher {
    fn matches(&self, statement: &str) -> bool;
    fn extract_parameters(&self, statement: &str) -> Option<HashMap<String, String>>;
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
}
