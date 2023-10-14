use std::collections::HashMap;
use std::io::{self, Write};
use std::cell::RefCell;
use std::rc::Rc;

const PROMPT: &str = "[cmd-rs] ";

// Define a trait for command handling
pub trait CommandHandler {
    fn execute(&self, cmd: Rc<RefCell<Cmd>>, args: String);
}

#[derive(Clone)]
pub struct Cmd {
    prompt: String,
    pub intro: Option<String>,
    commands: HashMap<String, Rc<dyn CommandHandler>>,
}

impl Cmd {
    pub fn new() -> Cmd {
        Cmd {
            prompt: PROMPT.to_string(),
            intro: None,
            commands: HashMap::new(),
        }
    }

    pub fn cmdloop(&self) {
        if let Some(ref intro) = self.intro {
            println!("{}", intro);
        }

        loop {
            print!("{}", self.prompt);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            let (cmd, args) = self.parse_line(&input);
            let cmd = cmd.to_string();
            if let Some(handler) = self.commands.get(&cmd) {
                handler.execute(Rc::new(RefCell::new(self.clone())), args);
            } else {
                println!("Unknown command: {}", cmd);
            }
        }
    }

    fn parse_line(&self, line: &str) -> (String, String) {
        let mut words = line.split_whitespace();
        let cmd = words.next().unwrap_or("").to_string();
        let args = words.collect::<Vec<_>>().join(" ");
        (cmd, args)
    }

    pub fn add_command(&mut self, name: &str, handler: Rc<dyn CommandHandler>) {
        self.commands.insert(name.to_string(), handler);
    }
}