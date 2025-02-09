use crate::cli::GenerateArgs;
use crate::generators::api::entity::EntityGenerator;
use crate::utils::errors::Error;

pub struct GenerateCommand;

impl GenerateCommand {
    pub fn run(args: GenerateArgs) -> Result<(), Error> {
        match args {
            GenerateArgs::Entity {
                name,
                fields,
                relations,
            } => {
                let parsed_fields = fields.map(|f| f.split(',').map(|s| s.to_string()).collect());
                let parsed_relations =
                    relations.map(|r| r.split(',').map(|s| s.to_string()).collect());
                EntityGenerator::new(name, parsed_fields, parsed_relations).generate()
            }
            GenerateArgs::Handler { name } => unimplemented!(),
        }
    }
}
