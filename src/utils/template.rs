use super::errors::Error;
use toml::Value;

pub fn load_template(template_path: &str) -> Result<String, Error> {
    println!("🔍 Loading template: {}", template_path);

    let template = std::fs::read_to_string(template_path)?;

    let parsed_toml: Value = toml::from_str(&template).map_err(|e| {
        Error::InvalidOptions(format!(
            "Invalid TOML structure in '{}': {}",
            template_path, e
        ))
    })?;

    parsed_toml
        .get("content")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| {
            Error::InvalidOptions(format!("Missing 'content' key in '{}'", template_path))
        })
}

pub fn replace_placeholders(template: &str, replacements: &[(&str, &str)]) -> String {
    let mut content = template.to_string();
    for (placeholder, value) in replacements {
        content = content.replace(&format!("{{{{{}}}}}", placeholder), value);
    }
    content
}
