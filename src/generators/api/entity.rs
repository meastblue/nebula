use colored::Colorize;

use crate::template;
use crate::types::ProjectType;
use crate::utils::{self, errors::Error};
use std::fs;

#[derive(Debug, Clone)]
enum RelationType {
    HasOne(String),
    HasMany(String),
    BelongsTo(String),
}

#[derive(Debug, Clone)]
pub struct FieldValidation {
    required: bool,
    min_length: Option<usize>,
    max_length: Option<usize>,
    pattern: Option<String>,
    min: Option<String>,
    max: Option<String>,
    unique: bool,
    email: bool,
    url: bool,
}

impl Default for FieldValidation {
    fn default() -> Self {
        Self {
            required: false,
            min_length: None,
            max_length: None,
            pattern: None,
            min: None,
            max: None,
            unique: false,
            email: false,
            url: false,
        }
    }
}

pub struct EntityGenerator {
    name: String,
    fields: Option<Vec<String>>,
}

impl EntityGenerator {
    pub fn new(name: String, fields: Option<Vec<String>>) -> Self {
        Self { name, fields }
    }

    pub fn generate(&self) -> Result<(), Error> {
        utils::tools::check_is_nebula_project()?;

        let parsed_fields = match &self.fields {
            Some(fields) => Self::parse_fields_and_relations(fields.clone())?,
            None => return Err(Error::InvalidOptions("Fields are required".to_string())),
        };

        let (fields_str, relations) = parsed_fields;
        let fields_str = fields_str
            .iter()
            .map(|(name, type_, _)| format!("{}: {}", name, type_))
            .collect::<Vec<_>>()
            .join(", ");
        let entity_content = template::get_entity_template(&self.name, &fields_str);

        let project_config = utils::tools::get_project_config()?;
        let entity_path = match project_config {
            ProjectType::Api => "src/entities",
            ProjectType::Full => "api/src/entities",
            _ => return Err(Error::InvalidOptions("Invalid project type".to_string())),
        };

        let file_name = format!("{}.rs", self.name.to_lowercase());
        utils::file::create_file_in_dir(entity_path, &file_name, &entity_content)?;

        println!("✅ Generated entity: {}", self.name);
        Ok(())
    }

    fn parse_fields_and_relations(
        fields: Vec<String>,
    ) -> Result<(Vec<(String, String, FieldValidation)>, Vec<RelationType>), Error> {
        let mut parsed_fields = vec![];
        let mut relations = vec![];

        for field in fields {
            let field_defs = field.split(',').collect::<Vec<_>>();

            for fdef in field_defs {
                if fdef.contains("->") {
                    let parts = fdef.split("->").collect::<Vec<_>>();

                    if parts.len() != 2 {
                        return Err(Error::InvalidRelationFormat(fdef.to_string()));
                    }

                    let relation_type = match parts[0].trim() {
                        "hasOne" => RelationType::HasOne(parts[1].trim().to_string()),
                        "hasMany" => RelationType::HasMany(parts[1].trim().to_string()),
                        "belongsTo" => RelationType::BelongsTo(parts[1].trim().to_string()),
                        _ => return Err(Error::InvalidRelationType(parts[0].trim().to_string())),
                    };

                    relations.push(relation_type);
                }

                if fdef.contains(":") {
                    let parts = fdef.split(':').collect::<Vec<_>>();

                    if parts.len() != 2 {
                        return Err(Error::InvalidFieldFormat(fdef.to_string()));
                    }

                    let name = parts[0].trim().to_string();
                    let type_and_validations: Vec<&str> = parts[1].split('|').collect();

                    if type_and_validations.is_empty() {
                        return Err(Error::MissingTypForField(name));
                    }

                    let field_type = type_and_validations[0].trim().to_string();
                    Self::validate_field_type(&field_type)?;

                    let mut validation = FieldValidation::default();

                    if type_and_validations.len() > 1 {
                        let validation_rules = type_and_validations[1]
                            .split(|c| c == ' ')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .collect::<Vec<_>>();

                        Self::parse_validations(&mut validation, &validation_rules)?;
                    }

                    parsed_fields.push((name, field_type, validation));
                }
            }
        }

        Ok((parsed_fields, relations))
    }

    fn parse_validations(validation: &mut FieldValidation, rules: &[&str]) -> Result<(), Error> {
        for rule in rules {
            let rule = rule.trim();
            if rule.is_empty() {
                continue;
            }

            let (rule_name, rule_value) = if let Some(idx) = rule.find('=') {
                let (name, value) = rule.split_at(idx);
                (name.trim(), Some(value[1..].trim()))
            } else {
                (rule, None)
            };

            match rule_name {
                "required" => validation.required = true,
                "unique" => validation.unique = true,
                "email" => validation.email = true,
                "url" => validation.url = true,
                "minLength" | "min_length" => {
                    if let Some(value) = rule_value {
                        let length = value.parse::<usize>().map_err(|_| {
                            Error::ValidationError(format!("Invalid minLength value: {}", value))
                        })?;
                        validation.min_length = Some(length);
                    } else {
                        return Err(Error::ValidationError("minLength requires a value".into()));
                    }
                }
                "maxLength" | "max_length" => {
                    if let Some(value) = rule_value {
                        let length = value.parse::<usize>().map_err(|_| {
                            Error::ValidationError(format!("Invalid maxLength value: {}", value))
                        })?;
                        validation.max_length = Some(length);
                    } else {
                        return Err(Error::ValidationError("maxLength requires a value".into()));
                    }
                }
                "min" => {
                    if let Some(value) = rule_value {
                        validation.min = Some(value.to_string());
                    } else {
                        return Err(Error::ValidationError("min requires a value".into()));
                    }
                }
                "max" => {
                    if let Some(value) = rule_value {
                        validation.max = Some(value.to_string());
                    } else {
                        return Err(Error::ValidationError("max requires a value".into()));
                    }
                }
                "pattern" => {
                    if let Some(value) = rule_value {
                        validation.pattern = Some(value.to_string());
                    } else {
                        return Err(Error::ValidationError("pattern requires a value".into()));
                    }
                }
                _ => {
                    return Err(Error::ValidationError(format!(
                        "Unknown validation rule: {}",
                        rule_name
                    )))
                }
            }
        }
        Ok(())
    }

    fn validate_field_type(field_type: &str) -> Result<(), Error> {
        let valid_types = [
            "String",
            "i32",
            "i64",
            "f32",
            "f64",
            "bool",
            "DateTime",
            "Vec<String>",
            "Option<String>",
            "u32",
            "u64",
            "usize",
        ];

        if !valid_types.contains(&field_type) {
            return Err(Error::ValidationError(format!(
                "Invalid type: {}. Valid types are: {}",
                field_type,
                valid_types.join(", ")
            )));
        }
        Ok(())
    }

    fn generate_entity_content(
        &self,
        template: &str,
        fields: &[(String, String, FieldValidation)],
        relations: &[RelationType],
    ) -> Result<String, Error> {
        let mut content = template.to_string();

        let fields_content = fields
            .iter()
            .map(|(name, type_, validation)| {
                let mut validations = vec![];
                if validation.required {
                    validations.push("#[validate(required)]".to_string());
                }
                if validation.unique {
                    validations.push("#[validate(custom = \"validate_unique\")]".to_string());
                }
                if validation.email {
                    validations.push("#[validate(email)]".to_string());
                }
                if validation.url {
                    validations.push("#[validate(url)]".to_string());
                }
                if let Some(min) = &validation.min {
                    validations.push(format!("#[validate(range(min = \"{}\"))]", min));
                }
                if let Some(max) = &validation.max {
                    validations.push(format!("#[validate(range(max = \"{}\"))]", max));
                }
                if let Some(min_length) = validation.min_length {
                    validations.push(format!("#[validate(length(min = {}))]", min_length));
                }
                if let Some(max_length) = validation.max_length {
                    validations.push(format!("#[validate(length(max = {}))]", max_length));
                }
                if let Some(pattern) = &validation.pattern {
                    validations.push(format!("#[validate(regex(path = \"{}\"))]", pattern));
                }

                format!(
                    "    {}\n    pub {}: {}",
                    validations.join("\n    "),
                    name,
                    type_
                )
            })
            .collect::<Vec<_>>()
            .join(",\n");

        content = content.replace("{fields}", &fields_content);

        // Replace placeholders for relations
        let relations_content = relations
            .iter()
            .map(|relation| match relation {
                RelationType::HasOne(target) => {
                    format!("    pub {}: Option<{}>", target.to_lowercase(), target)
                }
                RelationType::HasMany(target) => {
                    format!("    pub {}: Vec<{}>", target.to_lowercase(), target)
                }
                RelationType::BelongsTo(target) => {
                    format!(
                        "    pub {}_id: ID,\n    pub {}: Option<{}>",
                        target.to_lowercase(),
                        target.to_lowercase(),
                        target
                    )
                }
            })
            .collect::<Vec<_>>()
            .join(",\n");

        content = content.replace("{relations}", &relations_content);

        Ok(content)
    }

    fn update_entities_mod(&self, relations: &[RelationType]) -> Result<(), Error> {
        let mod_path = "backend/src/entities/mod.rs";
        let mut content = fs::read_to_string(mod_path).unwrap_or_else(|_| String::new());

        let module_name = self.name.to_lowercase();
        if !content.contains(&format!("pub mod {};", module_name)) {
            content.push_str(&format!("pub mod {};\n", module_name));
            content.push_str(&format!("pub use {}::{};\n", module_name, self.name));
        }

        for relation in relations {
            match relation {
                RelationType::HasOne(target)
                | RelationType::HasMany(target)
                | RelationType::BelongsTo(target) => {
                    if !content.contains(&format!("pub mod {};", target.to_lowercase())) {
                        content.push_str(&format!("pub mod {};\n", target.to_lowercase()));
                        content.push_str(&format!(
                            "pub use {}::{};\n",
                            target.to_lowercase(),
                            target
                        ));
                    }
                }
            }
        }

        fs::write(mod_path, content).map_err(|e| Error::FileSystem(e))?;
        Ok(())
    }
}
