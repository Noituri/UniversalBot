mod dev;
pub mod main;
mod moderation;
mod tickets;

use crate::command::Command;
use crate::config::DEV_MODULE;
use crate::utils::db::ServerInfo;

pub const PROTECTED_MODULES: [&str; 2] = ["main", DEV_MODULE];

pub trait BotModule {
    fn name(&self) -> String;
    fn desc(&self) -> String;
    fn commands(&self) -> Vec<Box<dyn Command>>;
}

impl dyn BotModule {
    pub fn enabled(&self, info: &ServerInfo) -> bool {
        match &info.server {
            Some(server) => server.enabledmodules.contains(&self.name()) || PROTECTED_MODULES.contains(&self.name().as_str()),
            None => true
        }

    }
}

pub fn get_modules() -> Vec<Box<dyn BotModule>> {
    vec![
        Box::new(main::MainModule {}),
        Box::new(moderation::ModerationModule {}),
        Box::new(tickets::TicketsModule {}),
        Box::new(dev::DevModule {}),
    ]
}

pub fn find_module(name: &str) -> Result<Box<dyn BotModule>, String> {
    for m in get_modules() {
        if m.name() == name.to_lowercase() {
            return Ok(m);
        }
    }

    Err(String::from("Module does not exist!"))
}
