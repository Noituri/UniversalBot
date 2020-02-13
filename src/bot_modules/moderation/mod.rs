use super::BotModule;
use crate::command::Command;

pub struct ModerationModule;

impl BotModule for ModerationModule {
    fn name(&self) -> String {
        String::from("moderation")
    }

    fn desc(&self) -> String {
        String::from("moderation commands.")
    }

    fn enabled(&self) -> bool {
        true
    }

    fn commands(&self) -> Vec<Box<dyn Command>> {
        vec![]
    }
}
