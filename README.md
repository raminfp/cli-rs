# cli-rs

#### What is line-oriented command interpreters?

A line-oriented command interpreter, also known as a command-line interpreter or shell, is a software program or interface that allows users to interact with a computer or operating system by entering text-based commands. Users type commands and arguments in a sequential, line-by-line manner, and the interpreter processes these commands and carries out the requested operations.


### Example

```rust

use std::cell::RefCell;
use std::rc::Rc;
use cmd_rs::cli::cmd::CommandHandler;
use cmd_rs::cli::cmd::Cmd;
use cmd_rs::about;


struct HelloCommand;
struct QuitCommand;
struct HelpCommand;


impl CommandHandler for HelloCommand {
    fn execute(&self, _cmd: Rc<RefCell<Cmd>>, args: String) {
        println!("Hello {}", args);
    }
}

impl CommandHandler for HelpCommand {
    fn execute(&self, _cmd: Rc<RefCell<Cmd>>, _args: String) {
        println!("Help");
    }
}


impl CommandHandler for QuitCommand {
    fn execute(&self, _cmd: Rc<RefCell<Cmd>>, _args: String) {
        std::process::exit(0);
    }
}


fn main() {
    let mut cmd = Cmd::new();
    cmd.intro = Some("Welcome to Rust command interpreter!".to_string());

    cmd.add_command("hello", Rc::new(HelloCommand));
    cmd.add_command("quit", Rc::new(QuitCommand));
    cmd.add_command("help", Rc::new(HelpCommand));
    cmd.add_command("about", Rc::new(about::AboutCommand));

    cmd.cmdloop();
}

```