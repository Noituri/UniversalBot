use super::BotModule;
use crate::command::Command;

mod ban_command;
mod unban_command;
mod kick_command;
mod mute_command;
mod unmute_command;
mod warn_command;
mod modtools_command;

pub struct ModerationModule;

impl BotModule for ModerationModule {
    fn name(&self) -> String {
        String::from("moderation")
    }

    fn desc(&self) -> String {
        String::from("Moderation commands.")
    }

    fn commands(&self) -> Vec<Box<dyn Command>> {
        vec![
            Box::new(ban_command::BanCommand {}),
            Box::new(unban_command::UnBanCommand {}),
            Box::new(kick_command::KickCommand {}),
            Box::new(mute_command::MuteCommand {}),
            Box::new(unmute_command::UnMuteCommand {}),
            Box::new(warn_command::WarnCommand {}),
        ]
    }
}
