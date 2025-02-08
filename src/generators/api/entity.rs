use crate::{
    template,
    types::ProjectType,
    utils::{self, errors::Error},
};
use colored::Colorize;
use std::fs;

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
    email: bool,
    url: bool,
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

        let parsed = match &self.fields {
            Some(fields) => Self::parse_fields(fields.clone())?,
            None => return Err(Error::InvalidOptions("Fields required".into())),
        };

        let (fields, relations) = parsed;
        let fields_str = fields
            .iter()
            .map(|(name, type_, _)| format!("{}: {}", name, type_))
            .collect::<Vec<_>>()
            .join(", ");

        let content = template::get_entity_template(&self.name, &fields_str);
        let path = self.get_entity_path()?;
        let filename = format!("{}.rs", self.name.to_lowercase());

        utils::file::create_file_in_dir(&path, &filename, &content)?;

        println!("✅ Generated entity: {}", self.name);
        Ok(())
    }

    fn get_entity_path(&self) -> Result<String, Error> {
        Ok(match utils::tools::get_project_config()? {
            ProjectType::Api => "src/entities".into(),
            ProjectType::Full => "api/src/entities".into(),
            _ => return Err(Error::InvalidOptions("Invalid project type".into())),
        })
    }

    fn parse_fields(
        fields: Vec<String>,
    ) -> Result<(Vec<(String, String, FieldRules)>, Vec<Relation>), Error> {
        let mut parsed = vec![];
        let mut relations = vec![];

        for field in fields {
            let defs = field.split(',').collect::<Vec<_>>();

            for def in defs {
                if def.contains("->") {
                    Self::parse_relation(def, &mut relations)?;
                }

                if def.contains(":") {
                    Self::parse_field(def, &mut parsed)?;
                }
            }
        }

        Ok((parsed, relations))
    }

    fn parse_relation(def: &str, relations: &mut Vec<Relation>) -> Result<(), Error> {
        let parts: Vec<_> = def.split("->").collect();

        if parts.len() != 2 {
            return Err(Error::InvalidRelationFormat(def.into()));
        }

        let relation = match parts[0].trim() {
            "hasOne" => Relation::HasOne(parts[1].trim().into()),
            "hasMany" => Relation::HasMany(parts[1].trim().into()),
            "belongsTo" => Relation::BelongsTo(parts[1].trim().into()),
            _ => return Err(Error::InvalidRelationType(parts[0].trim().into())),
        };

        relations.push(relation);
        Ok(())
    }

    fn parse_field(def: &str, fields: &mut Vec<(String, String, FieldRules)>) -> Result<(), Error> {
        let parts: Vec<_> = def.split(':').collect();

        if parts.len() != 2 {
            return Err(Error::InvalidFieldFormat(def.into()));
        }

        let name = parts[0].trim().into();
        let type_rules: Vec<_> = parts[1].split('|').collect();

        if type_rules.is_empty() {
            return Err(Error::MissingTypForField(name));
        }

        let field_type = type_rules[0].trim().to_string();
        Self::validate_field_type(&field_type)?;

        let mut rules = FieldRules::default();

        if type_rules.len() > 1 {
            Self::parse_rules(&mut rules, &type_rules[1..])?;
        }

        fields.push((name, field_type, rules));
        Ok(())
    }

    fn parse_rules(rules: &mut FieldRules, raw_rules: &[&str]) -> Result<(), Error> {
        for rule in raw_rules.iter().map(|s| s.trim()).filter(|s| !s.is_empty()) {
            let (name, value) = if let Some(i) = rule.find('=') {
                let (n, v) = rule.split_at(i);
                (n.trim(), Some(&v[1..]))
            } else {
                (rule, None)
            };

            match name {
                "required" => rules.required = true,
                "unique" => rules.unique = true,
                "email" => rules.email = true,
                "url" => rules.url = true,
                "minLength" | "min_length" => {
                    rules.min_length = Self::parse_length(value, "minLength")?
                }
                "maxLength" | "max_length" => {
                    rules.max_length = Self::parse_length(value, "maxLength")?
                }
                "min" => rules.min = value.map(String::from),
                "max" => rules.max = value.map(String::from),
                "pattern" => rules.pattern = value.map(String::from),
                _ => return Err(Error::ValidationError(format!("Unknown rule: {}", name))),
            }
        }
        Ok(())
    }

    fn parse_length(value: Option<&str>, name: &str) -> Result<Option<usize>, Error> {
        match value {
            Some(v) => Ok(Some(v.parse().map_err(|_| {
                Error::ValidationError(format!("Invalid {} value: {}", name, v))
            })?)),
            None => Err(Error::ValidationError(format!("{} needs value", name))),
        }
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
                "Invalid type: {}. Valid types: {}",
                field_type,
                valid_types.join(", ")
            )));
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
        if self.email {
            attrs.push("#[validate(email)]".into());
        }
        if self.url {
            attrs.push("#[validate(url)]".into());
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
