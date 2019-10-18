mod about_command;
mod modules_command;

use super::BotModule;
use crate::command::Command;

pub struct MainModule;

impl BotModule for MainModule {
    fn name(&self) -> String {
        String::from("main")
    }

    fn desc(&self) -> String {
        String::from("main module that provides basic commands for this bot.")
    }

    fn enabled(&self) -> bool {
        true
    }

    fn commands(&self) -> Vec<Box<dyn Command>> {
        vec! [
            Box::new(about_command::AboutCommand{}),
            Box::new(modules_command::ModulesCommand{})
        ]
    }
}