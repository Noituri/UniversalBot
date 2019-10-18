use log::{info, error};
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::error::Error;

pub struct Handler;

impl Handler {
    fn send_error(&self, ctx: Context, msg: Message, why: &str) {
        msg.channel_id.send_message(ctx.http, |m| {
            m.embed(|e| {
                e.title("Error");
                e.color(super::command::EMBED_ERROR_COLOR);
                e.description(why);
                e
            });
            m
        });
    }
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        // ignore other bots
        if msg.author.bot {
            return;
        }

        for m in super::get_modules().iter() {
            if !m.enabled() {
                continue;
            }

            for c in m.commands().iter() {
                if !c.enabled() {
                    continue;
                }

                if msg.content.starts_with(&format!("{}{}", super::config::PREFIX, c.name())) {
                    if !c.use_in_dm() && msg.is_private() {
                        self.send_error(ctx.clone(), msg.clone(), "This command is disabled in DM chat!");
                        return;
                    }
                    if let Err(why) = c.exe(&ctx, &msg) {
                        error!("Command '{}' failed. Reason: {}", c.name(), why.to_owned());
                        self.send_error(ctx.clone(), msg.clone(), &why);
                    }
                    break;
                }
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}