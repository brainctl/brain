use crate::core::{Command, CommandRegistry};

pub struct TaskCommand {
    registry: CommandRegistry,
}

impl TaskCommand {
    pub fn new() -> TaskCommand {
        let mut registry = CommandRegistry::new("task".to_string());
        registry.add_subcommand("add".to_string(), Box::new(AddCommand)); // Assuming AddCommand is another struct implementing Command

        TaskCommand { registry }
    }
}

impl Command for TaskCommand {
    fn execute(&self, args: &[String]) -> Result<(), String> {
        println!("Execting 'task' with args {:?}", args);
        self.registry.execute(args)
    }
}

struct AddCommand;
impl Command for AddCommand {
    fn execute(&self, args: &[String]) -> Result<(), String> {
        println!("Executing 'add' with args {:?}", args);
        Ok(())
    }
}