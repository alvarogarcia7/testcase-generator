use crate::models::{Verification, VerificationExpression};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Template category for organizing verification patterns
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TemplateCategory {
    /// HTTP status code verification
    HttpStatus,
    /// Exit code verification
    ExitCode,
    /// String matching patterns
    StringMatching,
    /// JSON validation patterns
    JsonValidation,
    /// Regex pattern matching
    RegexPatterns,
    /// Custom user-defined patterns
    Custom,
}

impl TemplateCategory {
    /// Get display name for the category
    pub fn display_name(&self) -> &str {
        match self {
            TemplateCategory::HttpStatus => "HTTP Status Codes",
            TemplateCategory::ExitCode => "Exit Codes",
            TemplateCategory::StringMatching => "String Matching",
            TemplateCategory::JsonValidation => "JSON Validation",
            TemplateCategory::RegexPatterns => "Regex Patterns",
            TemplateCategory::Custom => "Custom",
        }
    }
}

/// A verification template with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationTemplate {
    /// Template identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Detailed description
    pub description: String,
    /// Category this template belongs to
    pub category: TemplateCategory,
    /// The verification expression for result
    pub result_expression: String,
    /// The verification expression for output
    pub output_expression: String,
    /// Example usage scenarios
    pub examples: Vec<String>,
    /// Variables that can be substituted in the template
    pub variables: Vec<String>,
}

impl VerificationTemplate {
    /// Create a new verification template
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        category: TemplateCategory,
        result_expression: impl Into<String>,
        output_expression: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            category,
            result_expression: result_expression.into(),
            output_expression: output_expression.into(),
            examples: Vec::new(),
            variables: Vec::new(),
        }
    }

    /// Add an example to the template
    pub fn with_example(mut self, example: impl Into<String>) -> Self {
        self.examples.push(example.into());
        self
    }

    /// Add a variable to the template
    pub fn with_variable(mut self, variable: impl Into<String>) -> Self {
        self.variables.push(variable.into());
        self
    }

    /// Expand the template with variable substitutions
    pub fn expand(&self, substitutions: &HashMap<String, String>) -> Verification {
        let mut result = self.result_expression.clone();
        let mut output = self.output_expression.clone();

        for (var, value) in substitutions {
            let placeholder = format!("${{{}}}", var);
            result = result.replace(&placeholder, value);
            output = output.replace(&placeholder, value);
        }

        Verification {
            result: VerificationExpression::Simple(result),
            output: VerificationExpression::Simple(output),
            output_file: None,
        }
    }

    /// Expand the template without substitutions (use as-is)
    pub fn expand_default(&self) -> Verification {
        Verification {
            result: VerificationExpression::Simple(self.result_expression.clone()),
            output: VerificationExpression::Simple(self.output_expression.clone()),
            output_file: None,
        }
    }
}

/// Library of pre-built verification templates
pub struct VerificationTemplateLibrary {
    templates: HashMap<String, VerificationTemplate>,
}

impl VerificationTemplateLibrary {
    /// Create a new template library with pre-built templates
    pub fn new() -> Self {
        let mut library = Self {
            templates: HashMap::new(),
        };

        // HTTP Status Code Templates
        library.add_template(
            VerificationTemplate::new(
                "http_success",
                "HTTP Success (2xx)",
                "Verify HTTP response has success status code (200-299)",
                TemplateCategory::HttpStatus,
                "[[ $HTTP_STATUS -ge 200 && $HTTP_STATUS -lt 300 ]]",
                "cat $COMMAND_OUTPUT | grep -q \"${OUTPUT}\"",
            )
            .with_example("Verify API endpoint returns 200 OK")
            .with_variable("OUTPUT"),
        );

        library.add_template(
            VerificationTemplate::new(
                "http_200",
                "HTTP 200 OK",
                "Verify HTTP response is exactly 200 OK",
                TemplateCategory::HttpStatus,
                "[[ $HTTP_STATUS -eq 200 ]]",
                "cat $COMMAND_OUTPUT | grep -q \"${OUTPUT}\"",
            )
            .with_example("Verify successful GET request")
            .with_variable("OUTPUT"),
        );

        library.add_template(
            VerificationTemplate::new(
                "http_201",
                "HTTP 201 Created",
                "Verify HTTP response is 201 Created",
                TemplateCategory::HttpStatus,
                "[[ $HTTP_STATUS -eq 201 ]]",
                "cat $COMMAND_OUTPUT | grep -q \"${OUTPUT}\"",
            )
            .with_example("Verify resource creation")
            .with_variable("OUTPUT"),
        );

        library.add_template(
            VerificationTemplate::new(
                "http_204",
                "HTTP 204 No Content",
                "Verify HTTP response is 204 No Content",
                TemplateCategory::HttpStatus,
                "[[ $HTTP_STATUS -eq 204 ]]",
                "[[ -z \"$COMMAND_OUTPUT\" ]]",
            )
            .with_example("Verify DELETE operation with no content"),
        );

        library.add_template(
            VerificationTemplate::new(
                "http_400",
                "HTTP 400 Bad Request",
                "Verify HTTP response is 400 Bad Request",
                TemplateCategory::HttpStatus,
                "[[ $HTTP_STATUS -eq 400 ]]",
                "cat $COMMAND_OUTPUT | grep -q \"${OUTPUT}\"",
            )
            .with_example("Verify invalid request handling")
            .with_variable("OUTPUT"),
        );

        library.add_template(
            VerificationTemplate::new(
                "http_404",
                "HTTP 404 Not Found",
                "Verify HTTP response is 404 Not Found",
                TemplateCategory::HttpStatus,
                "[[ $HTTP_STATUS -eq 404 ]]",
                "cat $COMMAND_OUTPUT | grep -q \"${OUTPUT}\"",
            )
            .with_example("Verify resource not found")
            .with_variable("OUTPUT"),
        );

        // Exit Code Templates
        library.add_template(
            VerificationTemplate::new(
                "exit_success",
                "Exit Code 0 (Success)",
                "Verify command exits with success code 0",
                TemplateCategory::ExitCode,
                "[[ $? -eq 0 ]]",
                "cat $COMMAND_OUTPUT | grep -q \"${OUTPUT}\"",
            )
            .with_example("Verify successful command execution")
            .with_variable("OUTPUT"),
        );

        library.add_template(
            VerificationTemplate::new(
                "exit_failure",
                "Exit Code Non-Zero (Failure)",
                "Verify command exits with any non-zero code",
                TemplateCategory::ExitCode,
                "[[ $? -ne 0 ]]",
                "cat $COMMAND_OUTPUT | grep -q \"${OUTPUT}\"",
            )
            .with_example("Verify expected failure")
            .with_variable("OUTPUT"),
        );

        library.add_template(
            VerificationTemplate::new(
                "exit_code_custom",
                "Exit Code (Custom)",
                "Verify command exits with specific exit code",
                TemplateCategory::ExitCode,
                "[[ $? -eq ${EXIT_CODE} ]]",
                "cat $COMMAND_OUTPUT | grep -q \"${OUTPUT}\"",
            )
            .with_example("Verify specific exit code like 127 for command not found")
            .with_variable("EXIT_CODE")
            .with_variable("OUTPUT"),
        );

        // String Matching Templates
        library.add_template(
            VerificationTemplate::new(
                "string_exact",
                "Exact String Match",
                "Verify output exactly matches expected string",
                TemplateCategory::StringMatching,
                "[[ \"$RESULT\" == \"${EXPECTED}\" ]]",
                "[[ \"$COMMAND_OUTPUT\" == \"${OUTPUT}\" ]]",
            )
            .with_example("Verify exact output match")
            .with_variable("EXPECTED")
            .with_variable("OUTPUT"),
        );

        library.add_template(
            VerificationTemplate::new(
                "string_contains",
                "String Contains",
                "Verify output contains expected substring",
                TemplateCategory::StringMatching,
                "[[ \"$RESULT\" == *\"${EXPECTED}\"* ]]",
                "cat $COMMAND_OUTPUT | grep -q \"${OUTPUT}\"",
            )
            .with_example("Verify output contains specific text")
            .with_variable("EXPECTED")
            .with_variable("OUTPUT"),
        );

        library.add_template(
            VerificationTemplate::new(
                "string_starts_with",
                "String Starts With",
                "Verify output starts with expected prefix",
                TemplateCategory::StringMatching,
                "[[ \"$RESULT\" == \"${EXPECTED}\"* ]]",
                "cat $COMMAND_OUTPUT | grep -q \"^${OUTPUT}\"",
            )
            .with_example("Verify output prefix")
            .with_variable("EXPECTED")
            .with_variable("OUTPUT"),
        );

        library.add_template(
            VerificationTemplate::new(
                "string_ends_with",
                "String Ends With",
                "Verify output ends with expected suffix",
                TemplateCategory::StringMatching,
                "[[ \"$RESULT\" == *\"${EXPECTED}\" ]]",
                "cat $COMMAND_OUTPUT | grep -q \"${OUTPUT}$\"",
            )
            .with_example("Verify output suffix")
            .with_variable("EXPECTED")
            .with_variable("OUTPUT"),
        );

        library.add_template(
            VerificationTemplate::new(
                "string_empty",
                "Empty String",
                "Verify output is empty",
                TemplateCategory::StringMatching,
                "[[ -z \"$RESULT\" ]]",
                "[[ -z \"$COMMAND_OUTPUT\" ]]",
            )
            .with_example("Verify no output produced"),
        );

        library.add_template(
            VerificationTemplate::new(
                "string_not_empty",
                "Non-Empty String",
                "Verify output is not empty",
                TemplateCategory::StringMatching,
                "[[ -n \"$RESULT\" ]]",
                "[[ -n \"$COMMAND_OUTPUT\" ]]",
            )
            .with_example("Verify some output was produced"),
        );

        // JSON Validation Templates
        library.add_template(
            VerificationTemplate::new(
                "json_valid",
                "Valid JSON",
                "Verify output is valid JSON",
                TemplateCategory::JsonValidation,
                "echo \"$RESULT\" | jq empty",
                "cat $COMMAND_OUTPUT | jq empty",
            )
            .with_example("Verify API returns valid JSON"),
        );

        library.add_template(
            VerificationTemplate::new(
                "json_field_exists",
                "JSON Field Exists",
                "Verify JSON output contains a specific field",
                TemplateCategory::JsonValidation,
                "echo \"$RESULT\" | jq -e '.${FIELD}' > /dev/null",
                "cat $COMMAND_OUTPUT | jq -e '.${FIELD}' > /dev/null",
            )
            .with_example("Verify JSON has 'status' field")
            .with_variable("FIELD"),
        );

        library.add_template(
            VerificationTemplate::new(
                "json_field_value",
                "JSON Field Value",
                "Verify JSON field has expected value",
                TemplateCategory::JsonValidation,
                "echo \"$RESULT\" | jq -e '.${FIELD} == \"${VALUE}\"' > /dev/null",
                "cat $COMMAND_OUTPUT | jq -e '.${FIELD} == \"${VALUE}\"' > /dev/null",
            )
            .with_example("Verify JSON field 'status' equals 'success'")
            .with_variable("FIELD")
            .with_variable("VALUE"),
        );

        library.add_template(
            VerificationTemplate::new(
                "json_array_length",
                "JSON Array Length",
                "Verify JSON array has expected length",
                TemplateCategory::JsonValidation,
                "echo \"$RESULT\" | jq -e '.${FIELD} | length == ${LENGTH}' > /dev/null",
                "cat $COMMAND_OUTPUT | jq -e 'length == ${LENGTH}' > /dev/null",
            )
            .with_example("Verify array has 3 elements")
            .with_variable("FIELD")
            .with_variable("LENGTH"),
        );

        // Regex Pattern Templates
        library.add_template(
            VerificationTemplate::new(
                "regex_match",
                "Regex Pattern Match",
                "Verify output matches regular expression",
                TemplateCategory::RegexPatterns,
                "echo \"$RESULT\" | grep -qE \"${PATTERN}\"",
                "cat $COMMAND_OUTPUT | grep -qE \"${PATTERN}\"",
            )
            .with_example("Verify output matches pattern")
            .with_variable("PATTERN"),
        );

        library.add_template(
            VerificationTemplate::new(
                "regex_email",
                "Email Address",
                "Verify output contains valid email address",
                TemplateCategory::RegexPatterns,
                "echo \"$RESULT\" | grep -qE \"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}\"",
                "cat $COMMAND_OUTPUT | grep -qE \"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}\"",
            )
            .with_example("Verify output contains email address"),
        );

        library.add_template(
            VerificationTemplate::new(
                "regex_ipv4",
                "IPv4 Address",
                "Verify output contains valid IPv4 address",
                TemplateCategory::RegexPatterns,
                "echo \"$RESULT\" | grep -qE \"([0-9]{1,3}\\.){3}[0-9]{1,3}\"",
                "cat $COMMAND_OUTPUT | grep -qE \"([0-9]{1,3}\\.){3}[0-9]{1,3}\"",
            )
            .with_example("Verify output contains IP address"),
        );

        library.add_template(
            VerificationTemplate::new(
                "regex_uuid",
                "UUID",
                "Verify output contains valid UUID",
                TemplateCategory::RegexPatterns,
                "echo \"$RESULT\" | grep -qiE \"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\"",
                "cat $COMMAND_OUTPUT | grep -qiE \"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\"",
            )
            .with_example("Verify output contains UUID"),
        );

        library.add_template(
            VerificationTemplate::new(
                "regex_hexadecimal",
                "Hexadecimal Pattern",
                "Verify output matches hexadecimal pattern",
                TemplateCategory::RegexPatterns,
                "echo \"$RESULT\" | grep -qiE \"^(0x)?[0-9a-f]+$\"",
                "cat $COMMAND_OUTPUT | grep -qiE \"(0x)?[0-9a-f]+\"",
            )
            .with_example("Verify hex value like 0x9000"),
        );

        library
    }

    /// Add a template to the library
    pub fn add_template(&mut self, template: VerificationTemplate) {
        self.templates.insert(template.id.clone(), template);
    }

    /// Get a template by ID
    pub fn get_template(&self, id: &str) -> Option<&VerificationTemplate> {
        self.templates.get(id)
    }

    /// Get all templates
    pub fn get_all_templates(&self) -> Vec<&VerificationTemplate> {
        self.templates.values().collect()
    }

    /// Get templates by category
    pub fn get_templates_by_category(
        &self,
        category: &TemplateCategory,
    ) -> Vec<&VerificationTemplate> {
        self.templates
            .values()
            .filter(|t| &t.category == category)
            .collect()
    }

    /// Get all template IDs sorted alphabetically
    pub fn get_template_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.templates.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// Get template names for display (formatted as "name - description")
    pub fn get_template_display_names(&self) -> Vec<String> {
        let mut templates: Vec<_> = self.templates.values().collect();
        templates.sort_by_key(|t| (&t.category, &t.name));

        templates
            .iter()
            .map(|t| format!("{} - {}", t.name, t.description))
            .collect()
    }

    /// Get template from display name
    pub fn get_template_by_display_name(
        &self,
        display_name: &str,
    ) -> Option<&VerificationTemplate> {
        // Extract name from "name - description" format
        let name = display_name.split(" - ").next()?;
        self.templates.values().find(|t| t.name == name)
    }

    /// Get all categories
    pub fn get_categories(&self) -> Vec<TemplateCategory> {
        let mut categories: Vec<_> = self
            .templates
            .values()
            .map(|t| t.category.clone())
            .collect();
        categories.sort_by_key(|c| c.display_name().to_string());
        categories.dedup();
        categories
    }
}

impl Default for VerificationTemplateLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_creation() {
        let template = VerificationTemplate::new(
            "test_id",
            "Test Template",
            "A test template",
            TemplateCategory::ExitCode,
            "[[ $? -eq 0 ]]",
            "cat $COMMAND_OUTPUT",
        );

        assert_eq!(template.id, "test_id");
        assert_eq!(template.name, "Test Template");
        assert_eq!(template.category, TemplateCategory::ExitCode);
    }

    #[test]
    fn test_template_expansion_with_variables() {
        let template = VerificationTemplate::new(
            "test",
            "Test",
            "Test template",
            TemplateCategory::StringMatching,
            "[[ \"$RESULT\" == \"${EXPECTED}\" ]]",
            "cat $COMMAND_OUTPUT | grep -q \"${OUTPUT}\"",
        )
        .with_variable("EXPECTED")
        .with_variable("OUTPUT");

        let mut substitutions = HashMap::new();
        substitutions.insert("EXPECTED".to_string(), "success".to_string());
        substitutions.insert("OUTPUT".to_string(), "completed".to_string());

        let verification = template.expand(&substitutions);
        assert_eq!(
            verification.result,
            VerificationExpression::Simple("[[ \"$RESULT\" == \"success\" ]]".to_string())
        );
        assert_eq!(
            verification.output,
            VerificationExpression::Simple(
                "cat $COMMAND_OUTPUT | grep -q \"completed\"".to_string()
            )
        );
    }

    #[test]
    fn test_template_expansion_default() {
        let template = VerificationTemplate::new(
            "test",
            "Test",
            "Test template",
            TemplateCategory::ExitCode,
            "[[ $? -eq 0 ]]",
            "cat $COMMAND_OUTPUT",
        );

        let verification = template.expand_default();
        assert_eq!(
            verification.result,
            VerificationExpression::Simple("[[ $? -eq 0 ]]".to_string())
        );
        assert_eq!(
            verification.output,
            VerificationExpression::Simple("cat $COMMAND_OUTPUT".to_string())
        );
    }

    #[test]
    fn test_library_initialization() {
        let library = VerificationTemplateLibrary::new();
        assert!(!library.templates.is_empty());

        // Check some expected templates exist
        assert!(library.get_template("exit_success").is_some());
        assert!(library.get_template("http_200").is_some());
        assert!(library.get_template("json_valid").is_some());
        assert!(library.get_template("string_contains").is_some());
    }

    #[test]
    fn test_get_templates_by_category() {
        let library = VerificationTemplateLibrary::new();

        let http_templates = library.get_templates_by_category(&TemplateCategory::HttpStatus);
        assert!(!http_templates.is_empty());
        assert!(http_templates
            .iter()
            .all(|t| t.category == TemplateCategory::HttpStatus));

        let exit_templates = library.get_templates_by_category(&TemplateCategory::ExitCode);
        assert!(!exit_templates.is_empty());
        assert!(exit_templates
            .iter()
            .all(|t| t.category == TemplateCategory::ExitCode));
    }

    #[test]
    fn test_get_all_categories() {
        let library = VerificationTemplateLibrary::new();
        let categories = library.get_categories();

        assert!(categories.contains(&TemplateCategory::HttpStatus));
        assert!(categories.contains(&TemplateCategory::ExitCode));
        assert!(categories.contains(&TemplateCategory::StringMatching));
        assert!(categories.contains(&TemplateCategory::JsonValidation));
        assert!(categories.contains(&TemplateCategory::RegexPatterns));
    }

    #[test]
    fn test_template_with_example() {
        let template = VerificationTemplate::new(
            "test",
            "Test",
            "Test",
            TemplateCategory::Custom,
            "result",
            "output",
        )
        .with_example("Example 1")
        .with_example("Example 2");

        assert_eq!(template.examples.len(), 2);
        assert_eq!(template.examples[0], "Example 1");
        assert_eq!(template.examples[1], "Example 2");
    }

    #[test]
    fn test_category_display_name() {
        assert_eq!(
            TemplateCategory::HttpStatus.display_name(),
            "HTTP Status Codes"
        );
        assert_eq!(TemplateCategory::ExitCode.display_name(), "Exit Codes");
        assert_eq!(
            TemplateCategory::StringMatching.display_name(),
            "String Matching"
        );
        assert_eq!(
            TemplateCategory::JsonValidation.display_name(),
            "JSON Validation"
        );
        assert_eq!(
            TemplateCategory::RegexPatterns.display_name(),
            "Regex Patterns"
        );
    }

    #[test]
    fn test_http_status_templates() {
        let library = VerificationTemplateLibrary::new();

        let http_200 = library.get_template("http_200").unwrap();
        assert_eq!(http_200.name, "HTTP 200 OK");
        assert!(http_200.result_expression.contains("200"));

        let http_404 = library.get_template("http_404").unwrap();
        assert_eq!(http_404.name, "HTTP 404 Not Found");
        assert!(http_404.result_expression.contains("404"));
    }

    #[test]
    fn test_json_templates() {
        let library = VerificationTemplateLibrary::new();

        let json_valid = library.get_template("json_valid").unwrap();
        assert!(json_valid.result_expression.contains("jq"));

        let json_field = library.get_template("json_field_exists").unwrap();
        assert!(json_field.variables.contains(&"FIELD".to_string()));
    }

    #[test]
    fn test_regex_templates() {
        let library = VerificationTemplateLibrary::new();

        let email = library.get_template("regex_email").unwrap();
        assert!(email.result_expression.contains("grep"));
        assert!(email.result_expression.contains("@"));

        let ipv4 = library.get_template("regex_ipv4").unwrap();
        assert!(ipv4.result_expression.contains("grep"));
    }

    #[test]
    fn test_get_template_display_names() {
        let library = VerificationTemplateLibrary::new();
        let display_names = library.get_template_display_names();

        assert!(!display_names.is_empty());
        // Check format is "name - description"
        assert!(display_names.iter().any(|name| name.contains(" - ")));
    }

    #[test]
    fn test_get_template_by_display_name() {
        let library = VerificationTemplateLibrary::new();
        let display_names = library.get_template_display_names();

        let first_display = &display_names[0];
        let template = library.get_template_by_display_name(first_display);
        assert!(template.is_some());
    }
}
