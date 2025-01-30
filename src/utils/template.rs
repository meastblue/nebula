use std::path::PathBuf;
use toml::Value;

pub fn load_template(template_path: &str) -> Result<String, String> {
    let mut path = PathBuf::from(std::env::current_dir().map_err(|e| e.to_string())?);
    path.push(template_path);

    let template = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let template: Value = toml::from_str(&template).map_err(|e| e.to_string())?;
    Ok(template["content"].as_str().unwrap().to_string())
}

pub fn replace_placeholders(template: &str, replacements: &[(&str, &str)]) -> String {
    let mut content = template.to_string();
    for (placeholder, value) in replacements {
        content = content.replace(&format!("{{{{{}}}}}", placeholder), value);
    }
    content
}
