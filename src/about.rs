use std::cell::RefCell;
use std::rc::Rc;
use crate::cli::cmd::CommandHandler;
use crate::cli::cmd::Cmd;

pub struct AboutCommand;

impl CommandHandler for AboutCommand {
    fn execute(&self, _cmd: Rc<RefCell<Cmd>>, _args: String) {
        println!("About me");
    }
}

