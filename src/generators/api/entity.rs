use crate::{
    template,
    types::ProjectType,
    utils::{self, errors::Error},
};
use std::{fmt, iter, path::PathBuf};

#[derive(Debug, Clone)]
enum Relation {
    HasOne(String),
    HasMany(String),
    BelongsTo(String),
}

trait Validatable {
    fn validate(&self) -> Result<(), Error>;
}

#[derive(Debug, Clone, Default)]
pub struct FieldRules {
    required: bool,
    min_length: Option<usize>,
    max_length: Option<usize>,
    pattern: Option<String>,
    min: Option<String>,
    max: Option<String>,
    unique: bool,
}

#[derive(Debug, Clone, Default)]
pub struct FieldValidator {
    unique: bool,
    custom_rules: Vec<String>,
}

pub struct EntityField {
    name: String,
    field_type: String,
    validators: FieldValidator,
    relation: Option<Relation>,
}

impl std::fmt::Display for EntityField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.field_type)?;
        if !self.validators.custom_rules.is_empty() {
            write!(f, " [{}]", self.validators.custom_rules.join(", "))?;
        }
        Ok(())
    }
}

pub struct EntityGenerator {
    name: String,
    fields: Option<Vec<String>>,
    relations: Option<Vec<String>>,
}

impl EntityField {
    fn new(raw_field: &str) -> Result<Self, Error> {
        let mut parts = raw_field.splitn(2, '|');
        let name_type = parts
            .next()
            .ok_or(Error::InvalidOptions("Empty field".into()))?;
        let (name, field_type) = if name_type.contains(':') {
            let mut parts = name_type.splitn(2, ':');
            (parts.next().unwrap().trim(), parts.next().unwrap().trim())
        } else {
            let mut parts = name_type.splitn(2, '|');
            (
                parts.next().unwrap().trim(),
                parts.next().unwrap_or("String").trim(),
            )
        };
        let validators = parts
            .next()
            .unwrap_or_default()
            .split('|')
            .collect::<Vec<_>>();
        let relation = match field_type {
            s if s.starts_with("has_one:") => Some(Relation::HasOne(s[8..].to_string())),
            s if s.starts_with("has_many:") => Some(Relation::HasMany(s[9..].to_string())),
            s if s.starts_with("belongs_to:") => Some(Relation::BelongsTo(s[11..].to_string())),
            _ => None,
        };
        Ok(Self {
            name: name.to_string(),
            field_type: field_type.to_string(),
            validators: FieldValidator {
                unique: validators.contains(&"unique"),
                custom_rules: validators
                    .iter()
                    .filter(|&&v| v != "unique")
                    .map(|s| s.to_string())
                    .collect(),
            },
            relation,
        })
    }
}

impl EntityGenerator {
    pub fn new(name: String, fields: Option<Vec<String>>, relations: Option<Vec<String>>) -> Self {
        Self {
            name,
            fields,
            relations,
        }
    }

    pub fn generate(&self) -> Result<(), Error> {
        self.validate()?;
        utils::tools::check_is_nebula_project()?;
        let fields = self.parse_fields()?;
        let relations = self.parse_relations()?;
        let content = self.generate_content(&fields, &relations)?;
        self.write_entity_file(&content)?;
        self.update_mod_file()?;
        Ok(())
    }

    fn parse_fields(&self) -> Result<Vec<EntityField>, Error> {
        self.fields
            .as_ref()
            .map(|f| f.iter().map(|s| EntityField::new(s)).collect())
            .unwrap_or_else(|| Ok(Vec::new()))
    }

    fn parse_relations(&self) -> Result<Vec<EntityField>, Error> {
        self.relations
            .as_ref()
            .map(|r| r.iter().map(|s| EntityField::new(s)).collect())
            .unwrap_or_else(|| Ok(Vec::new()))
    }

    fn generate_content(
        &self,
        fields: &[EntityField],
        relations: &[EntityField],
    ) -> Result<String, Error> {
        let all_fields = fields.iter().chain(relations.iter());
        let fields_code = all_fields
            .map(|f| f.to_rust_code())
            .collect::<Vec<_>>()
            .join("\n");
        Ok(template::get_entity_template(&self.name, &fields_code))
    }

    fn get_base_path(&self) -> Result<PathBuf, Error> {
        match utils::tools::get_project_config()? {
            ProjectType::Api => Ok(PathBuf::from("src/")),
            ProjectType::Full => Ok(PathBuf::from("api/src/")),
            _ => Err(Error::InvalidOptions("Invalid project type".into())),
        }
    }

    fn write_entity_file(&self, content: &str) -> Result<(), Error> {
        let base_path = self.get_base_path()?;
        let entity_dir = base_path.join(&self.name.to_lowercase());
        std::fs::create_dir_all(&entity_dir)?;
        std::fs::write(entity_dir.join("entity.rs"), content)?;
        std::fs::write(
            entity_dir.join("mod.rs"),
            "mod entity;\npub use entity::*;\n",
        )?;
        Ok(())
    }

    fn update_mod_file(&self) -> Result<(), Error> {
        let mod_path = self.get_base_path()?.join("mod.rs");
        let mod_line = format!("pub mod {};", self.name.to_lowercase());
        let mut content = std::fs::read_to_string(&mod_path).unwrap_or_default();
        if !content.contains(&mod_line) {
            content.push_str(&format!("\n{}", mod_line));
            std::fs::write(mod_path, content)?;
        }
        Ok(())
    }
}

impl Validatable for EntityGenerator {
    fn validate(&self) -> Result<(), Error> {
        if self.name.is_empty() || !self.name.chars().all(|c| c.is_alphanumeric()) {
            return Err(Error::ValidationError("Invalid entity name".into()));
        }
        Ok(())
    }
}

impl FieldValidator {
    fn to_validation_attributes(&self) -> Vec<String> {
        let mut attrs = Vec::new();
        if self.unique {
            attrs.push("#[validate(custom = \"validate_unique\")]".into());
        }
        attrs.extend(
            self.custom_rules
                .iter()
                .map(|r| format!("#[validate({})]", r)),
        );
        attrs
    }
}

impl EntityField {
    fn to_rust_code(&self) -> String {
        let mut code = Vec::new();
        code.extend(
            self.validators
                .to_validation_attributes()
                .into_iter()
                .map(|a| format!("    {}", a)),
        );
        let field_type = match &self.relation {
            Some(Relation::HasOne(t)) => format!("Option<{}>", t),
            Some(Relation::HasMany(t)) => format!("Vec<{}>", t),
            Some(Relation::BelongsTo(t)) => t.clone(),
            None => self.field_type.clone(),
        };
        code.push(format!("    pub {}: {},", self.name, field_type));
        code.join("\n")
    }
}
