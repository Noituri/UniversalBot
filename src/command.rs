use serenity::model::channel::Message;
use std::error::Error;
use serenity::prelude::Context;

pub enum CommandPerms {
    ServerOwner,
    Ban
}

pub struct CommandConfig {
    pub name: String,
    pub values: Vec<String>
}

pub trait Command {
    fn name(&self) -> String;
    fn desc(&self) -> String;
    fn enabled(&self) -> bool;
    fn args(&self) -> Option<Vec<String>>;
    fn perms(&self) -> Option<Vec<CommandPerms>>;
    fn config(&self) -> Option<Vec<CommandConfig>>;
    fn exe(&self, ctx: Context,  msg: Message) -> Result<Message, Box<dyn Error>>;
}