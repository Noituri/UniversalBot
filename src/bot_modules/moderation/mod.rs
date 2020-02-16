use super::BotModule;
use crate::command::Command;

pub struct ModerationModule;

impl BotModule for ModerationModule {
    fn name(&self) -> String {
        String::from("moderation")
    }

    fn desc(&self) -> String {
        String::from("Moderation commands.")
    }

    fn commands(&self) -> Vec<Box<dyn Command>> {
        vec![]
    }
}
