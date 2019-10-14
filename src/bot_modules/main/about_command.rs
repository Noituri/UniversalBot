use crate::command::{Command, CommandPerms, CommandConfig};
use serenity::model::channel::Message;
use std::error::Error;
use serenity::prelude::Context;

pub struct AboutCommand;

impl Command for AboutCommand {
    fn name(&self) -> String {
        String::from("about")
    }

    fn desc(&self) -> String {
        String::from("shows information about this bot.")
    }

    fn enabled(&self) -> bool {
        true
    }

    fn args(&self) -> Option<Vec<String>> {
        None
    }

    fn perms(&self) -> Option<Vec<CommandPerms>> {
        None
    }

    fn config(&self) -> Option<Vec<CommandConfig>> {
        None
    }

    fn exe(&self, ctx: Context, msg: Message) -> Result<Message, Box<dyn Error>> {
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("BO2T");
                e.description("pong");
                e
            });
            m
        });
        Ok(msg)
    }
}