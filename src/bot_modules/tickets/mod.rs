use super::BotModule;
use crate::command::Command;

mod new_command;
mod solved_command;

pub struct TicketsModule;

impl BotModule for TicketsModule {
    fn name(&self) -> String {
        String::from("tickets")
    }

    fn desc(&self) -> String {
        String::from("Tickets commands")
    }

    fn commands(&self) -> Vec<Box<dyn Command>> {
        vec![
            Box::new(new_command::NewTicketCommand {}),
            Box::new(solved_command::SolvedTicketCommand {}),
        ]
    }
}
