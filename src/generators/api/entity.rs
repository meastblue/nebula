use crate::{
    template,
    types::ProjectType,
    utils::{self, errors::Error},
};
use std::path::PathBuf;

#[derive(Debug, Clone)]
enum Relation {
    HasOne(String),
    HasMany(String),
    BelongsTo(String),
}

trait Validatable {
    fn validate(&self) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct FieldRules {
    required: bool,
    min_length: Option<usize>,
    max_length: Option<usize>,
    pattern: Option<String>,
    min: Option<String>,
    max: Option<String>,
    unique: bool,
}

impl Default for FieldRules {
    fn default() -> Self {
        Self {
            required: false,
            min_length: None,
            max_length: None,
            pattern: None,
            min: None,
            max: None,
            unique: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldValidator {
    unique: bool,
    email: bool,
    url: bool,
}

impl Default for FieldValidator {
    fn default() -> Self {
        Self {
            unique: false,
            email: false,
            url: false,
        }
    }
}

pub struct EntityField {
    name: String,
    field_type: String,
    validators: FieldValidator,
}

impl EntityField {
    fn new(raw: &str) -> Result<Self, Error> {
        let parts: Vec<&str> = raw.split('|').collect();
        let (name, field_type) = Self::parse_name_type(parts[0])?;
        let validators = Self::parse_validators(parts.get(1..).unwrap_or(&[]));

        Ok(Self {
            name,
            field_type,
            validators,
        })
    }

    fn parse_name_type(raw: &str) -> Result<(String, String), Error> {
        let parts: Vec<&str> = raw.split(':').collect();
        match (parts.get(0), parts.get(1)) {
            (Some(&name), Some(&type_)) => Ok((name.trim().to_string(), type_.trim().to_string())),
            _ => Err(Error::InvalidOptions("Invalid field format".into())),
        }
    }

    fn parse_validators(validators: &[&str]) -> FieldValidator {
        let mut validator = FieldValidator::default();
        for &v in validators {
            match v {
                "unique" => validator.unique = true,
                "email" => validator.email = true,
                "url" => validator.url = true,
                _ => {}
            }
        }
        validator
    }

    fn to_string(&self) -> String {
        format!("{}: {}", self.name, self.field_type)
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

        let fields = self.parse_fields()?;
        let content = self.generate_content(&fields)?;
        self.write_entity_file(&content)?;
        self.update_mod_file()?;

        println!("✅ Generated entity: {}", self.name);
        Ok(())
    }

    fn parse_fields(&self) -> Result<Vec<EntityField>, Error> {
        self.fields
            .as_ref()
            .ok_or_else(|| Error::InvalidOptions("Fields required".into()))?
            .iter()
            .map(|f| EntityField::new(f))
            .collect()
    }

    fn generate_content(&self, fields: &[EntityField]) -> Result<String, Error> {
        let fields_str = fields
            .iter()
            .map(EntityField::to_string)
            .collect::<Vec<_>>()
            .join(",\n    ");

        Ok(template::get_entity_template(&self.name, &fields_str))
    }

    fn get_base_path(&self) -> Result<PathBuf, Error> {
        let base = match utils::tools::get_project_config()? {
            ProjectType::Api => "src/",
            ProjectType::Full => "api/src/",
            _ => return Err(Error::InvalidOptions("Invalid project type".into())),
        };
        Ok(PathBuf::from(base))
    }

    fn write_entity_file(&self, content: &str) -> Result<(), Error> {
        let base_path = self.get_base_path()?;
        let entity_dir = base_path.join(&self.name.to_lowercase());
        std::fs::create_dir_all(&entity_dir).map_err(Error::FileSystem)?;

        let entity_file = entity_dir.join("entity.rs");
        std::fs::write(&entity_file, content).map_err(Error::FileSystem)?;

        let mod_content = format!("mod entity;\npub use entity::*;\n");
        let mod_file = entity_dir.join("mod.rs");
        std::fs::write(&mod_file, mod_content).map_err(Error::FileSystem)?;

        Ok(())
    }

    fn update_mod_file(&self) -> Result<(), Error> {
        let base_path = self.get_base_path()?;
        let mod_path = base_path.join("mod.rs");
        let mod_line = format!("pub mod {};", self.name.to_lowercase());

        let content = if !mod_path.exists() {
            mod_line
        } else {
            let mut existing = std::fs::read_to_string(&mod_path).map_err(Error::FileSystem)?;
            if !existing.contains(&mod_line) {
                existing.push_str(&format!("\n{}", mod_line));
            }
            existing
        };

        std::fs::write(mod_path, content).map_err(Error::FileSystem)
    }
}

impl FieldRules {
    fn to_validation_attributes(&self) -> Vec<String> {
        let mut attrs = Vec::new();

        if self.required {
            attrs.push("#[validate(required)]".into());
        }
        if self.unique {
            attrs.push("#[validate(custom = \"validate_unique\")]".into());
        }

        self.add_range_validations(&mut attrs);
        self.add_length_validations(&mut attrs);

        if let Some(p) = &self.pattern {
            attrs.push(format!("#[validate(regex(path = \"{}\"))]", p));
        }
        attrs
    }

    fn add_range_validations(&self, attrs: &mut Vec<String>) {
        if let Some(min) = &self.min {
            attrs.push(format!("#[validate(range(min = \"{}\"))]", min));
        }
        if let Some(max) = &self.max {
            attrs.push(format!("#[validate(range(max = \"{}\"))]", max));
        }
    }

    fn add_length_validations(&self, attrs: &mut Vec<String>) {
        if let Some(min) = self.min_length {
            attrs.push(format!("#[validate(length(min = {}))]", min));
        }
        if let Some(max) = self.max_length {
            attrs.push(format!("#[validate(length(max = {}))]", max));
        }
    }
}
