use crate::{
    template,
    types::ProjectType,
    utils::{self, errors::Error},
};
use std::{iter, path::PathBuf};

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
    fields: Option<String>,
    relations: Option<String>,
}

impl EntityField {
    fn new(raw_field: &str) -> Result<Self, Error> {
        let mut parts = raw_field.splitn(2, '|');
        let name_type = parts
            .next()
            .ok_or(Error::InvalidOptions("Empty field".into()))?;

        let (name, field_type) = if name_type.contains(':') {
            let mut name_type_parts = name_type.splitn(2, ':');
            match (name_type_parts.next(), name_type_parts.next()) {
                (Some(name), Some(typ)) => (name.trim().to_string(), typ.trim().to_string()),
                _ => {
                    return Err(Error::InvalidOptions(format!(
                        "Invalid field format: '{}'",
                        name_type
                    )))
                }
            }
        } else {
            let mut name_type_parts = name_type.splitn(2, '|');
            match (name_type_parts.next(), name_type_parts.next()) {
                (Some(name), Some(typ)) => (name.trim().to_string(), typ.trim().to_string()),
                _ => {
                    return Err(Error::InvalidOptions(format!(
                        "Invalid field format: '{}'",
                        name_type
                    )))
                }
            }
        };

        let validators = parts
            .next()
            .map(|s| s.split('|').collect::<Vec<_>>())
            .unwrap_or_default();

        let relation = if field_type.starts_with("has_one:") {
            Some(Relation::HasOne(field_type["has_one:".len()..].to_string()))
        } else if field_type.starts_with("has_many:") {
            Some(Relation::HasMany(
                field_type["has_many:".len()..].to_string(),
            ))
        } else if field_type.starts_with("belongs_to:") {
            Some(Relation::BelongsTo(
                field_type["belongs_to:".len()..].to_string(),
            ))
        } else {
            None
        };

        Ok(Self {
            name,
            field_type,
            validators: Self::parse_validators(&validators)?,
            relation,
        })
    }

    fn parse_validators(rules: &[&str]) -> Result<FieldValidator, Error> {
        let mut validator = FieldValidator::default();
        for rule in rules {
            match *rule {
                "unique" => validator.unique = true,
                custom => validator.custom_rules.push(custom.to_string()),
            }
        }
        Ok(validator)
    }

    fn to_rust_code(&self) -> String {
        let mut code = Vec::new();

        let attrs = self.validators.to_validation_attributes();
        code.extend(attrs.into_iter().map(|a| format!("    {}", a)));

        let field_type = match &self.relation {
            Some(Relation::HasOne(target)) => format!("Option<{}>", target),
            Some(Relation::HasMany(target)) => format!("Vec<{}>", target),
            Some(Relation::BelongsTo(target)) => target.clone(),
            None => self.field_type.clone(),
        };

        code.push(format!("    pub {}: {},", self.name, field_type));

        code.join("\n")
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

impl EntityGenerator {
    pub fn new(name: String, fields: Option<String>, relations: Option<String>) -> Self {
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
        println!("✅ Generated entity: {}", self.name);
        Ok(())
    }

    fn parse_fields(&self) -> Result<Vec<EntityField>, Error> {
        match &self.fields {
            Some(fields) if !fields.is_empty() => fields
                .split(',')
                .map(|field_str| EntityField::new(field_str))
                .collect(),
            _ => Ok(Vec::new()),
        }
    }

    fn parse_relations(&self) -> Result<Vec<EntityField>, Error> {
        match &self.relations {
            Some(relations) if !relations.is_empty() => relations
                .split(',')
                .map(|relation_str| EntityField::new(relation_str))
                .collect(),
            _ => Ok(Vec::new()),
        }
    }

    fn generate_content(
        &self,
        fields: &[EntityField],
        relations: &[EntityField],
    ) -> Result<String, Error> {
        let fields_repr = if !fields.is_empty() || !relations.is_empty() {
            let all_fields = fields.iter().chain(relations.iter());
            all_fields
                .map(|f| f.to_rust_code())
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            "// Add your fields here".to_string()
        };

        Ok(template::get_entity_template(&self.name, &fields_repr))
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

        let mut content = if mod_path.exists() {
            std::fs::read_to_string(&mod_path)?
        } else {
            String::new()
        };

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
            return Err(Error::ValidationError(
                "Entity name must be non-empty and alphanumeric".into(),
            ));
        }

        if let Some(fields) = &self.fields {
            if fields.split(',').any(|f| f.trim().is_empty()) {
                return Err(Error::ValidationError(
                    "Field definitions cannot be empty".into(),
                ));
            }
        }

        if let Some(relations) = &self.relations {
            if relations.split(',').any(|r| r.trim().is_empty()) {
                return Err(Error::ValidationError(
                    "Relation definitions cannot be empty".into(),
                ));
            }
        }

        Ok(())
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
        iter::once(&self.min)
            .chain(iter::once(&self.max))
            .enumerate()
            .for_each(|(i, val)| {
                if let Some(v) = val {
                    let bound = if i == 0 { "min" } else { "max" };
                    attrs.push(format!("#[validate(range({} = \"{}\"))]", bound, v));
                }
            });
    }

    fn add_length_validations(&self, attrs: &mut Vec<String>) {
        iter::once(self.min_length)
            .chain(iter::once(self.max_length))
            .enumerate()
            .for_each(|(i, val)| {
                if let Some(v) = val {
                    let bound = if i == 0 { "min" } else { "max" };
                    attrs.push(format!("#[validate(length({} = {}))]", bound, v));
                }
            });
    }
}
