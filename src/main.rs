use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead, Write};
use std::process;

type CommandCallback = fn(&CommandRegistry) -> Result<(), Box<dyn Error>>;

struct CliCommand {
    name: &'static str,
    description: &'static str,
    callback: CommandCallback,
}

struct CommandRegistry {
    commands: HashMap<&'static str, CliCommand>,
}

impl CommandRegistry {
    fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    fn register(&mut self, name: &'static str, description: &'static str, callback: CommandCallback) {
        self.commands.insert(
            name,
            CliCommand {
                name,
                description,
                callback,
            },
        );
    }

    fn get(&self, name: &str) -> Option<&CliCommand> {
        self.commands.get(name)
    }

    fn init() -> Self {
        let mut registry = Self::new();
        registry.register("exit", "Exit the Pokedex", command_exit);
        registry.register("help", "Displays a help message", command_help);
        registry
    }
}

// 1. Parameter _registry hinzugefügt
fn command_exit(_registry: &CommandRegistry) -> Result<(), Box<dyn Error>> {
    println!("Closing the Pokedex... Goodbye!");
    process::exit(0);
}

// 2. Nutzt direkt die übergebene Registry statt get_commands()
fn command_help(registry: &CommandRegistry) -> Result<(), Box<dyn Error>> {
    println!();
    println!("Welcome to the Pokedex!");
    println!("Usage:\n");

    for cmd in registry.commands.values() {
        println!("{}: {}", cmd.name, cmd.description);
    }

    println!();
    Ok(())
}

fn clean_input(text: &str) -> Vec<String> {
    text.trim()
        .split_whitespace()
        .map(|s| s.to_lowercase())
        .collect()
}

fn read_input() {
    // 3. CommandRegistry initialisieren statt get_commands()
    let registry = CommandRegistry::init();
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        print!("Pokedex > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if reader.read_line(&mut input).unwrap() == 0 {
            break;
        }

        let cleaned_input = clean_input(&input);

        if let Some(command_name) = cleaned_input.first() {
            if let Some(cmd) = registry.get(command_name.as_str()) {
                if let Err(err) = (cmd.callback)(&registry) {
                    eprintln!("Error executing command '{}': {}", cmd.name, err);
                }
            } else {
                println!("Unknown command");
            }
        }
    }
}

fn main() {
    read_input();
}