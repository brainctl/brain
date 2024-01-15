use std::collections::HashMap;

pub trait Command {
    fn execute(&self, args: &[String]) -> Result<(), String>;
}

pub struct CommandRegistry {
    name: String,
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandRegistry {
    pub fn new(name: String) -> CommandRegistry {
        CommandRegistry {
            name,
            commands: HashMap::new(),
        }
    }

    pub fn add_subcommand(&mut self, name: String, subcommand: Box<dyn Command>) {
        self.commands.insert(name, subcommand);
    }

    pub fn get_subcommand(&self, name: &str) -> Option<&Box<dyn Command>> {
        self.commands.get(name)
    }
}

impl Command for CommandRegistry {
    fn execute(&self, args: &[String]) -> Result<(), String> {
        if let Some((cmd_name, remaining_args)) = args.split_first() {
            if let Some(subcommand) = self.get_subcommand(cmd_name) {
                println!("Looking up '{}' in '{}' command registry with args {:?}", cmd_name, self.name, remaining_args);
                subcommand.execute(remaining_args)
            } else {
                Err(format!("Subcommand '{}' not found", cmd_name))
            }
        } else {
            Ok(())
        }
    }
}