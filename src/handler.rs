use log::{info, error};
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::error::Error;

pub struct Handler;

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
                    if let Err(why) = c.exe(&ctx, &msg) {
                        error!("Command '{}' failed", c.name());
                        msg.channel_id.send_message(ctx.clone().http, |m| {
                            m.embed(|e| {
                                e.title("Error");
                                e.color(super::command::EMBED_ERROR_COLOR);
                                e.description(why);
                                e
                            });
                            m
                        });
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