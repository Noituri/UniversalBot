use log::{info, error};
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::model::channel::Embed;

pub struct Handler;

impl Handler {
    fn handle_result<T>(&self, result: Result<(), T>) {
        if let Err(why) = result {

        }
    }
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        // ignore other bots
        if msg.author.bot {
            return;
        }

        for c in super::get_modules()[0].commands() {
            if msg.content.starts_with(&format!("{}{}", super::config::PREFIX, c.name())) {
                c.exe(ctx, msg);
                break;
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}