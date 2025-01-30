use super::errors::Error;
use toml::Value;

pub struct TemplateUtils;

impl TemplateUtils {
    /// Charge un template TOML et retourne son contenu
    pub fn load_template(template_path: &str) -> Result<String, Error> {
        let template = std::fs::read_to_string(template_path)?;
        let template: Value = toml::from_str(&template)?;
        Ok(template["content"].as_str().unwrap_or("").to_string())
    }

    /// Remplace les placeholders dans un template
    pub fn replace_placeholders(template: &str, replacements: &[(&str, &str)]) -> String {
        let mut content = template.to_string();
        for (placeholder, value) in replacements {
            content = content.replace(&format!("{{{{{}}}}}", placeholder), value);
        }
        content
    }
}
