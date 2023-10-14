use std::cell::RefCell;
use std::rc::Rc;
use crate::cli::cmd::CommandHandler;
use crate::cli::cmd::Cmd;

pub struct HelloCommand;

impl CommandHandler for HelloCommand {
    fn execute(&self, _cmd: Rc<RefCell<Cmd>>, args: String) {
        println!("Hello {}", args);
    }
}

