mod core;
mod command {
    pub mod task;
}

use core::{Command, CommandRegistry};
use command::task::TaskCommand;

struct GlobalFlags {
    config: String,
    daemonize: bool,
}

struct BrainInvocation {
    global_flags: GlobalFlags,
    command: Box<dyn Command>,
}

fn main() {
    let mut task_cmd = TaskCommand::new();

    let mut root_registry = CommandRegistry::new("brain".to_string());
    root_registry.add_subcommand("task".to_string(), Box::new(task_cmd));

    // Example usage
    let args = vec!["task".to_string(), "add".to_string(), "asd".to_string(), "--priority".to_string(), "3".to_string()];
    match root_registry.execute(&args) {
        Ok(()) => println!("Command executed successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
