use std::cell::RefCell;
use std::rc::Rc;
use cmd_rs::cli::cmd::CommandHandler;
use cmd_rs::cli::cmd::Cmd;
use cmd_rs::about;
use cmd_rs::hello;

struct QuitCommand;
struct HelpCommand;

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

    cmd.add_command("hello", Rc::new(hello::HelloCommand));
    cmd.add_command("quit", Rc::new(QuitCommand));
    cmd.add_command("help", Rc::new(HelpCommand));
    cmd.add_command("about", Rc::new(about::AboutCommand));

    cmd.cmdloop();
}
