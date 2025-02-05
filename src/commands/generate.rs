use crate::cli::GenerateArgs;
use crate::generators::api::entity::EntityGenerator;
use crate::utils::errors::Error;

pub struct GenerateCommand;

impl GenerateCommand {
    pub fn run(args: GenerateArgs) -> Result<(), Error> {
        match args {
            GenerateArgs::Entity { name, fields } => EntityGenerator::new(name, fields).generate(),
            GenerateArgs::Handler { name } => unimplemented!(),
        }
    }
}
