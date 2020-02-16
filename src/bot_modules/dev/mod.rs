use super::BotModule;
use crate::command::Command;

pub struct DevModule;

impl BotModule for DevModule {
    fn name(&self) -> String {
        String::from("dev")
    }

    fn desc(&self) -> String {
        String::from("provides commands that can be used by this bot developers.")
    }

    fn commands(&self) -> Vec<Box<dyn Command>> {
        vec![]
    }
}
