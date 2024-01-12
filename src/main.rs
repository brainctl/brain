use std::collections::HashMap;

struct GlobalFlags {
    config: String,
    daemonize: bool,
}

struct BrainInvocation {
    global_flags: GlobalFlags,
    command: Box<dyn BrainCommand>,
}

trait BrainCommand {
    fn execute(&self, args: &[String]) -> Result<(), String>;
    fn add_subcommand(&mut self, name: String, subcommand: Box<dyn BrainCommand>);
    fn get_subcommand(&self, name: &str) -> Option<&Box<dyn BrainCommand>>;
}

struct CommandRegistry {
    name: String,
    commands: HashMap<String, Box<dyn BrainCommand>>,
}

impl CommandRegistry {
    fn new(name: String) -> CommandRegistry {
        CommandRegistry {
            name: name,
            commands: HashMap::new(),
        }
    }

    fn add_subcommand(&mut self, name: String, subcommand: Box<dyn BrainCommand>) {
        self.commands.insert(name, subcommand);
    }

    fn get_subcommand(&self, name: &str) -> Option<&Box<dyn BrainCommand>> {
        self.commands.get(name)
    }
}

impl BrainCommand for CommandRegistry {
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

    fn add_subcommand(&mut self, name: String, subcommand: Box<dyn BrainCommand>) {
        self.add_subcommand(name, subcommand);
    }

    fn get_subcommand(&self, name: &str) -> Option<&Box<dyn BrainCommand>> {
        self.get_subcommand(name)
    }
}

struct TaskCommand {
    subcommands: CommandRegistry,
}

impl TaskCommand {
    fn new() -> TaskCommand {
        TaskCommand {
            subcommands: CommandRegistry::new("task".to_string()),
        }
    }
}

impl BrainCommand for TaskCommand {
    fn execute(&self, args: &[String]) -> Result<(), String> {
        println!("Execting 'task' with args {:?}", args);
        self.subcommands.execute(args)
    }

    fn add_subcommand(&mut self, name: String, subcommand: Box<dyn BrainCommand>) {
        self.subcommands.add_subcommand(name, subcommand);
    }

    fn get_subcommand(&self, name: &str) -> Option<&Box<dyn BrainCommand>> {
        self.subcommands.get_subcommand(name)
    }
}

struct AddCommand;

impl BrainCommand for AddCommand {
    fn execute(&self, args: &[String]) -> Result<(), String> {
        println!("Executing 'add' with args {:?}", args);
        Ok(())
    }

    fn add_subcommand(&mut self, name: String, subcommand: Box<dyn BrainCommand>) {
        // do nothing, I am a leaf command
    }

    fn get_subcommand(&self, name: &str) -> Option<&Box<dyn BrainCommand>> {
        // do nothing, I am a leaf command
        None
    }
}

fn main() {
    let mut task_cmd = TaskCommand::new();
    task_cmd.add_subcommand("add".to_string(), Box::new(AddCommand)); // Assuming AddCommand is another struct implementing BrainCommand

    let mut root_registry = CommandRegistry::new("brain".to_string());
    root_registry.add_subcommand("task".to_string(), Box::new(task_cmd));

    // Example usage
    let args = vec!["task".to_string(), "add".to_string(), "asd".to_string(), "--priority".to_string(), "3".to_string()];
    match root_registry.execute(&args) {
        Ok(()) => println!("Command executed successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
