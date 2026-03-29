use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Manages variable hydration for test case YAML files
#[derive(Debug, Clone)]
pub struct VarHydrator {
    variables: HashMap<String, String>,
}

impl VarHydrator {
    /// Create a new VarHydrator instance with empty variables
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Create a VarHydrator with predefined variables
    pub fn with_variables(variables: HashMap<String, String>) -> Self {
        Self { variables }
    }

    /// Load variables from a bash export file
    ///
    /// Parses lines in the format:
    /// ```bash
    /// export VAR_NAME=value
    /// export VAR_NAME="quoted value"
    /// export VAR_NAME='single quoted'
    /// ```
    ///
    /// # Arguments
    /// * `file_path` - Path to the bash export file
    ///
    /// # Returns
    /// * `Result<()>` - Ok if successful, Err with context on failure
    pub fn load_from_export_file<P: AsRef<Path>>(&mut self, file_path: P) -> Result<()> {
        let file_path_ref = file_path.as_ref();
        let content = fs::read_to_string(file_path_ref).context(format!(
            "Failed to read export file: {}",
            file_path_ref.display()
        ))?;

        // Pattern matches: export VAR_NAME=value or export VAR_NAME="value" or export VAR_NAME='value'
        let export_pattern = Regex::new(
            r#"^\s*export\s+([A-Za-z_][A-Za-z0-9_]*)=(?:"([^"]*)"|'([^']*)'|([^\s#]+))"#,
        )
        .context("Failed to compile export regex pattern")?;

        for line in content.lines() {
            // Skip empty lines and comments
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if let Some(caps) = export_pattern.captures(line) {
                let var_name = caps.get(1).unwrap().as_str().to_string();

                // Check which capture group matched: double quotes, single quotes, or unquoted
                let value = if let Some(m) = caps.get(2) {
                    m.as_str().to_string()
                } else if let Some(m) = caps.get(3) {
                    m.as_str().to_string()
                } else if let Some(m) = caps.get(4) {
                    m.as_str().to_string()
                } else {
                    String::new()
                };

                self.variables.insert(var_name, value);
            }
        }

        Ok(())
    }

    /// Extract all ${#VAR_NAME} placeholders from YAML content
    ///
    /// Finds all unique placeholders in the format ${#VAR_NAME} where VAR_NAME
    /// consists of uppercase letters, digits, and underscores.
    ///
    /// # Arguments
    /// * `yaml_content` - The YAML content to scan for placeholders
    ///
    /// # Returns
    /// * `Vec<String>` - Vector of unique variable names (without ${#} wrapper)
    pub fn extract_placeholders(&self, yaml_content: &str) -> Vec<String> {
        // Pattern matches ${#VAR_NAME} where VAR_NAME is uppercase letters, digits, underscores
        let placeholder_pattern = Regex::new(r"\$\{#([A-Z_][A-Z0-9_]*)\}").unwrap();

        let mut placeholders = std::collections::HashSet::new();

        for caps in placeholder_pattern.captures_iter(yaml_content) {
            if let Some(var_name) = caps.get(1) {
                placeholders.insert(var_name.as_str().to_string());
            }
        }

        let mut result: Vec<String> = placeholders.into_iter().collect();
        result.sort();
        result
    }

    /// Hydrate YAML content by replacing ${#VAR_NAME} placeholders with values
    ///
    /// Replaces all occurrences of ${#VAR_NAME} with the corresponding value
    /// from the loaded variables. If a variable is not found, the placeholder
    /// is left unchanged.
    ///
    /// # Arguments
    /// * `yaml_content` - The YAML content with placeholders
    ///
    /// # Returns
    /// * `String` - The hydrated YAML content
    pub fn hydrate_yaml_content(&self, yaml_content: &str) -> String {
        let placeholder_pattern = Regex::new(r"\$\{#([A-Z_][A-Z0-9_]*)\}").unwrap();

        placeholder_pattern
            .replace_all(yaml_content, |caps: &regex::Captures| {
                let var_name = &caps[1];
                if let Some(value) = self.variables.get(var_name) {
                    value.clone()
                } else {
                    // Leave placeholder unchanged if variable not found
                    caps[0].to_string()
                }
            })
            .to_string()
    }

    /// Generate a bash export file with current variables
    ///
    /// Creates a bash script with export statements in the format:
    /// ```bash
    /// export VAR_NAME=value
    /// ```
    ///
    /// Values are quoted if they contain spaces or special characters.
    ///
    /// # Arguments
    /// * `file_path` - Path where the export file should be written
    ///
    /// # Returns
    /// * `Result<()>` - Ok if successful, Err with context on failure
    pub fn generate_export_file<P: AsRef<Path>>(&self, file_path: P) -> Result<()> {
        let file_path_ref = file_path.as_ref();

        let mut lines: Vec<String> = Vec::new();

        // Sort variables by name for consistent output
        let mut sorted_vars: Vec<(&String, &String)> = self.variables.iter().collect();
        sorted_vars.sort_by_key(|(k, _)| *k);

        for (var_name, value) in sorted_vars {
            // Quote value if it contains spaces, special characters, or is empty
            let needs_quotes = value.is_empty()
                || value.contains(' ')
                || value.contains('\t')
                || value.contains('\n')
                || value.contains('"')
                || value.contains('\'')
                || value.contains('$')
                || value.contains('\\')
                || value.contains('`')
                || value.contains('!')
                || value.contains('&')
                || value.contains('|')
                || value.contains(';')
                || value.contains('<')
                || value.contains('>')
                || value.contains('(')
                || value.contains(')')
                || value.contains('[')
                || value.contains(']')
                || value.contains('{')
                || value.contains('}')
                || value.contains('*')
                || value.contains('?')
                || value.contains('#');

            let export_line = if needs_quotes {
                // Escape double quotes and backslashes in the value
                let escaped_value = value
                    .replace('\\', "\\\\")
                    .replace('"', "\\\"")
                    .replace('$', "\\$")
                    .replace('`', "\\`");
                format!("export {}=\"{}\"", var_name, escaped_value)
            } else {
                format!("export {}={}", var_name, value)
            };

            lines.push(export_line);
        }

        let content = lines.join("\n");
        let final_content = if content.is_empty() {
            String::new()
        } else {
            format!("{}\n", content)
        };

        fs::write(file_path_ref, final_content).context(format!(
            "Failed to write export file: {}",
            file_path_ref.display()
        ))?;

        Ok(())
    }

    /// Get all loaded variables
    pub fn variables(&self) -> &HashMap<String, String> {
        &self.variables
    }

    /// Get a specific variable value
    pub fn get(&self, var_name: &str) -> Option<&String> {
        self.variables.get(var_name)
    }

    /// Set a variable value
    pub fn set(&mut self, var_name: String, value: String) {
        self.variables.insert(var_name, value);
    }

    /// Remove a variable
    pub fn remove(&mut self, var_name: &str) -> Option<String> {
        self.variables.remove(var_name)
    }

    /// Clear all variables
    pub fn clear(&mut self) {
        self.variables.clear();
    }

    /// Check if a variable exists
    pub fn contains(&self, var_name: &str) -> bool {
        self.variables.contains_key(var_name)
    }

    /// Get the number of variables
    pub fn len(&self) -> usize {
        self.variables.len()
    }

    /// Check if there are no variables
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }
}

impl Default for VarHydrator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_new_var_hydrator() {
        let hydrator = VarHydrator::new();
        assert!(hydrator.is_empty());
        assert_eq!(hydrator.len(), 0);
    }

    #[test]
    fn test_with_variables() {
        let mut vars = HashMap::new();
        vars.insert("VAR1".to_string(), "value1".to_string());
        vars.insert("VAR2".to_string(), "value2".to_string());

        let hydrator = VarHydrator::with_variables(vars);
        assert_eq!(hydrator.len(), 2);
        assert_eq!(hydrator.get("VAR1"), Some(&"value1".to_string()));
        assert_eq!(hydrator.get("VAR2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_load_from_export_file_simple() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "export VAR1=value1")?;
        writeln!(file, "export VAR2=value2")?;

        let mut hydrator = VarHydrator::new();
        hydrator.load_from_export_file(file.path())?;

        assert_eq!(hydrator.len(), 2);
        assert_eq!(hydrator.get("VAR1"), Some(&"value1".to_string()));
        assert_eq!(hydrator.get("VAR2"), Some(&"value2".to_string()));
        Ok(())
    }

    #[test]
    fn test_load_from_export_file_quoted() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "export VAR1=\"quoted value\"")?;
        writeln!(file, "export VAR2='single quoted'")?;
        writeln!(file, "export VAR3=unquoted")?;

        let mut hydrator = VarHydrator::new();
        hydrator.load_from_export_file(file.path())?;

        assert_eq!(hydrator.len(), 3);
        assert_eq!(hydrator.get("VAR1"), Some(&"quoted value".to_string()));
        assert_eq!(hydrator.get("VAR2"), Some(&"single quoted".to_string()));
        assert_eq!(hydrator.get("VAR3"), Some(&"unquoted".to_string()));
        Ok(())
    }

    #[test]
    fn test_load_from_export_file_with_comments() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "# This is a comment")?;
        writeln!(file, "export VAR1=value1")?;
        writeln!(file)?;
        writeln!(file, "export VAR2=value2 # inline comment not supported")?;

        let mut hydrator = VarHydrator::new();
        hydrator.load_from_export_file(file.path())?;

        assert_eq!(hydrator.len(), 2);
        assert_eq!(hydrator.get("VAR1"), Some(&"value1".to_string()));
        assert_eq!(hydrator.get("VAR2"), Some(&"value2".to_string()));
        Ok(())
    }

    #[test]
    fn test_load_from_export_file_with_spaces() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "  export VAR1=value1  ")?;
        writeln!(file, "export  VAR2=value2")?;

        let mut hydrator = VarHydrator::new();
        hydrator.load_from_export_file(file.path())?;

        assert_eq!(hydrator.len(), 2);
        assert_eq!(hydrator.get("VAR1"), Some(&"value1".to_string()));
        assert_eq!(hydrator.get("VAR2"), Some(&"value2".to_string()));
        Ok(())
    }

    #[test]
    fn test_extract_placeholders() {
        let hydrator = VarHydrator::new();
        let yaml = r#"
            command: ssh ${#SERVER_HOST}
            output: ${#EXPECTED_OUTPUT}
            another: ${#SERVER_HOST}
            nested: "value with ${#NESTED_VAR} inside"
        "#;

        let placeholders = hydrator.extract_placeholders(yaml);
        assert_eq!(placeholders.len(), 3);
        assert!(placeholders.contains(&"SERVER_HOST".to_string()));
        assert!(placeholders.contains(&"EXPECTED_OUTPUT".to_string()));
        assert!(placeholders.contains(&"NESTED_VAR".to_string()));
    }

    #[test]
    fn test_extract_placeholders_no_matches() {
        let hydrator = VarHydrator::new();
        let yaml = "command: ssh server\noutput: result";

        let placeholders = hydrator.extract_placeholders(yaml);
        assert_eq!(placeholders.len(), 0);
    }

    #[test]
    fn test_extract_placeholders_various_formats() {
        let hydrator = VarHydrator::new();
        let yaml = r#"
            valid: ${#VAR_NAME}
            with_numbers: ${#VAR_123}
            underscore_start: ${#_VAR}
            all_caps: ${#ALLCAPS}
            invalid_lowercase: ${#lowercase}
            invalid_no_hash: ${VAR}
        "#;

        let placeholders = hydrator.extract_placeholders(yaml);
        // Only uppercase patterns with # should match
        assert!(placeholders.contains(&"VAR_NAME".to_string()));
        assert!(placeholders.contains(&"VAR_123".to_string()));
        assert!(placeholders.contains(&"_VAR".to_string()));
        assert!(placeholders.contains(&"ALLCAPS".to_string()));
        assert!(!placeholders.contains(&"lowercase".to_string()));
    }

    #[test]
    fn test_hydrate_yaml_content() {
        let mut vars = HashMap::new();
        vars.insert("SERVER_HOST".to_string(), "example.com".to_string());
        vars.insert("PORT".to_string(), "8080".to_string());

        let hydrator = VarHydrator::with_variables(vars);
        let yaml = "server: ${#SERVER_HOST}\nport: ${#PORT}";

        let hydrated = hydrator.hydrate_yaml_content(yaml);
        assert_eq!(hydrated, "server: example.com\nport: 8080");
    }

    #[test]
    fn test_hydrate_yaml_content_missing_var() {
        let mut vars = HashMap::new();
        vars.insert("VAR1".to_string(), "value1".to_string());

        let hydrator = VarHydrator::with_variables(vars);
        let yaml = "field1: ${#VAR1}\nfield2: ${#VAR2}";

        let hydrated = hydrator.hydrate_yaml_content(yaml);
        assert_eq!(hydrated, "field1: value1\nfield2: ${#VAR2}");
    }

    #[test]
    fn test_hydrate_yaml_content_multiple_occurrences() {
        let mut vars = HashMap::new();
        vars.insert("HOST".to_string(), "server1".to_string());

        let hydrator = VarHydrator::with_variables(vars);
        let yaml = "primary: ${#HOST}\nbackup: ${#HOST}\nmirror: ${#HOST}";

        let hydrated = hydrator.hydrate_yaml_content(yaml);
        assert_eq!(
            hydrated,
            "primary: server1\nbackup: server1\nmirror: server1"
        );
    }

    #[test]
    fn test_generate_export_file_simple() -> Result<()> {
        let mut vars = HashMap::new();
        vars.insert("VAR1".to_string(), "value1".to_string());
        vars.insert("VAR2".to_string(), "value2".to_string());

        let hydrator = VarHydrator::with_variables(vars);
        let temp_file = NamedTempFile::new()?;

        hydrator.generate_export_file(temp_file.path())?;

        let content = fs::read_to_string(temp_file.path())?;
        assert!(content.contains("export VAR1=value1"));
        assert!(content.contains("export VAR2=value2"));
        Ok(())
    }

    #[test]
    fn test_generate_export_file_quoted() -> Result<()> {
        let mut vars = HashMap::new();
        vars.insert("VAR1".to_string(), "value with spaces".to_string());
        vars.insert("VAR2".to_string(), "simple".to_string());

        let hydrator = VarHydrator::with_variables(vars);
        let temp_file = NamedTempFile::new()?;

        hydrator.generate_export_file(temp_file.path())?;

        let content = fs::read_to_string(temp_file.path())?;
        assert!(content.contains("export VAR1=\"value with spaces\""));
        assert!(content.contains("export VAR2=simple"));
        Ok(())
    }

    #[test]
    fn test_generate_export_file_special_chars() -> Result<()> {
        let mut vars = HashMap::new();
        vars.insert("VAR1".to_string(), "value with \"quotes\"".to_string());
        vars.insert("VAR2".to_string(), "value$with$dollar".to_string());
        vars.insert("VAR3".to_string(), "value\\with\\backslash".to_string());

        let hydrator = VarHydrator::with_variables(vars);
        let temp_file = NamedTempFile::new()?;

        hydrator.generate_export_file(temp_file.path())?;

        let content = fs::read_to_string(temp_file.path())?;
        assert!(content.contains("export VAR1=\"value with \\\"quotes\\\"\""));
        assert!(content.contains("export VAR2=\"value\\$with\\$dollar\""));
        assert!(content.contains("export VAR3=\"value\\\\with\\\\backslash\""));
        Ok(())
    }

    #[test]
    fn test_generate_export_file_sorted() -> Result<()> {
        let mut vars = HashMap::new();
        vars.insert("ZZZ".to_string(), "last".to_string());
        vars.insert("AAA".to_string(), "first".to_string());
        vars.insert("MMM".to_string(), "middle".to_string());

        let hydrator = VarHydrator::with_variables(vars);
        let temp_file = NamedTempFile::new()?;

        hydrator.generate_export_file(temp_file.path())?;

        let content = fs::read_to_string(temp_file.path())?;
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines[0], "export AAA=first");
        assert_eq!(lines[1], "export MMM=middle");
        assert_eq!(lines[2], "export ZZZ=last");
        Ok(())
    }

    #[test]
    fn test_generate_export_file_empty() -> Result<()> {
        let hydrator = VarHydrator::new();
        let temp_file = NamedTempFile::new()?;

        hydrator.generate_export_file(temp_file.path())?;

        let content = fs::read_to_string(temp_file.path())?;
        assert_eq!(content, "");
        Ok(())
    }

    #[test]
    fn test_set_and_get() {
        let mut hydrator = VarHydrator::new();
        hydrator.set("VAR1".to_string(), "value1".to_string());

        assert_eq!(hydrator.get("VAR1"), Some(&"value1".to_string()));
        assert_eq!(hydrator.get("VAR2"), None);
    }

    #[test]
    fn test_remove() {
        let mut hydrator = VarHydrator::new();
        hydrator.set("VAR1".to_string(), "value1".to_string());

        let removed = hydrator.remove("VAR1");
        assert_eq!(removed, Some("value1".to_string()));
        assert_eq!(hydrator.get("VAR1"), None);

        let removed_again = hydrator.remove("VAR1");
        assert_eq!(removed_again, None);
    }

    #[test]
    fn test_clear() {
        let mut hydrator = VarHydrator::new();
        hydrator.set("VAR1".to_string(), "value1".to_string());
        hydrator.set("VAR2".to_string(), "value2".to_string());

        assert_eq!(hydrator.len(), 2);
        hydrator.clear();
        assert_eq!(hydrator.len(), 0);
        assert!(hydrator.is_empty());
    }

    #[test]
    fn test_contains() {
        let mut hydrator = VarHydrator::new();
        hydrator.set("VAR1".to_string(), "value1".to_string());

        assert!(hydrator.contains("VAR1"));
        assert!(!hydrator.contains("VAR2"));
    }

    #[test]
    fn test_round_trip_export_and_load() -> Result<()> {
        let mut vars = HashMap::new();
        vars.insert("SERVER".to_string(), "example.com".to_string());
        vars.insert("PORT".to_string(), "8080".to_string());
        vars.insert("MESSAGE".to_string(), "Hello World".to_string());

        let hydrator1 = VarHydrator::with_variables(vars);
        let temp_file = NamedTempFile::new()?;

        hydrator1.generate_export_file(temp_file.path())?;

        let mut hydrator2 = VarHydrator::new();
        hydrator2.load_from_export_file(temp_file.path())?;

        assert_eq!(hydrator2.len(), 3);
        assert_eq!(hydrator2.get("SERVER"), Some(&"example.com".to_string()));
        assert_eq!(hydrator2.get("PORT"), Some(&"8080".to_string()));
        assert_eq!(hydrator2.get("MESSAGE"), Some(&"Hello World".to_string()));
        Ok(())
    }

    #[test]
    fn test_full_hydration_workflow() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "export SERVER_HOST=example.com")?;
        writeln!(file, "export SERVER_PORT=8080")?;
        writeln!(file, "export API_KEY=secret123")?;

        let mut hydrator = VarHydrator::new();
        hydrator.load_from_export_file(file.path())?;

        let yaml_template = r#"
server:
  host: ${#SERVER_HOST}
  port: ${#SERVER_PORT}
api:
  key: ${#API_KEY}
  endpoint: https://${#SERVER_HOST}/api
"#;

        let placeholders = hydrator.extract_placeholders(yaml_template);
        assert_eq!(placeholders.len(), 3);

        let hydrated = hydrator.hydrate_yaml_content(yaml_template);
        assert!(hydrated.contains("host: example.com"));
        assert!(hydrated.contains("port: 8080"));
        assert!(hydrated.contains("key: secret123"));
        assert!(hydrated.contains("endpoint: https://example.com/api"));

        Ok(())
    }

    #[test]
    fn test_default() {
        let hydrator = VarHydrator::default();
        assert!(hydrator.is_empty());
    }

    #[test]
    fn test_variables_accessor() {
        let mut vars = HashMap::new();
        vars.insert("VAR1".to_string(), "value1".to_string());

        let hydrator = VarHydrator::with_variables(vars.clone());
        assert_eq!(hydrator.variables(), &vars);
    }
}
