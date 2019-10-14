use log::{info, error};
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

pub struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        // ignore other bots
        if msg.author.bot {
            return;
        }
        
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
                error!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}