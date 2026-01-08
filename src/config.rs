use std::env;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct EditorConfig {
    pub editor: Option<String>,
    pub visual: Option<String>,
    pub custom_fallback: Option<String>,
}

impl EditorConfig {
    pub fn load() -> Self {
        let original_vars = Self::save_env_vars();
        
        if Path::new(".env").exists() {
            let _ = dotenvy::from_filename(".env");
        }
        
        if Path::new(".env.local").exists() {
            let _ = dotenvy::from_filename(".env.local");
        }
        
        let editor = Self::get_with_precedence("EDITOR", &original_vars);
        let visual = Self::get_with_precedence("VISUAL", &original_vars);
        let custom_fallback = Self::get_with_precedence("CUSTOM_FALLBACK", &original_vars);
        
        EditorConfig {
            editor,
            visual,
            custom_fallback,
        }
    }
    
    fn save_env_vars() -> (Option<String>, Option<String>, Option<String>) {
        (
            env::var("EDITOR").ok(),
            env::var("VISUAL").ok(),
            env::var("CUSTOM_FALLBACK").ok(),
        )
    }
    
    fn get_with_precedence(
        key: &str,
        original_vars: &(Option<String>, Option<String>, Option<String>),
    ) -> Option<String> {
        let original = match key {
            "EDITOR" => &original_vars.0,
            "VISUAL" => &original_vars.1,
            "CUSTOM_FALLBACK" => &original_vars.2,
            _ => &None,
        };
        
        if original.is_some() {
            return original.clone();
        }
        
        env::var(key).ok()
    }
    
    pub fn get_editor(&self) -> Option<String> {
        self.visual
            .as_ref()
            .or(self.editor.as_ref())
            .or(self.custom_fallback.as_ref())
            .cloned()
    }
}
