use log::{info, error};
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use crate::utils::{get_server, has_perms};

pub struct Handler;

impl Handler {
    fn send_error(&self, ctx: Context, msg: Message, why: &str) {
        let _ = msg.channel_id.send_message(ctx.http, |m| {
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

        let guild = get_server(msg.guild_id);
        let prefix =
            if msg.content.starts_with(&format!("<@{}> ", ctx.cache.read().user.id)) {
                format!("<@{}> ", ctx.cache.read().user.id)
            }
            else if let Some(g) = guild.to_owned() {
                g.prefix
            } else {
                super::config::DEFAULT_PREFIX.to_owned()
            };

        for m in super::get_modules().iter() {
            if !m.enabled() {
                continue;
            }

            for c in m.commands().iter() {
                if !c.enabled() {
                    continue;
                }

                if msg.content.starts_with(&format!("{}{}", prefix, c.name())) {
                    if !c.use_in_dm() && msg.is_private() {
                        self.send_error(ctx.clone(), msg.clone(), "This command is disabled in DM chat!");
                        return;
                    } else if !msg.is_private() {
                        if !has_perms(&ctx, &msg,guild.clone().unwrap(), &c.perms()) {
                            let mut needed_perms = String::new();
                            let mut command_example = format!("{}perms add <@role/role_name/role_id>", prefix);
                            c.perms().unwrap().iter().for_each(|p| {
                                let perm_name = p.to_uppercase();
                                needed_perms.push_str(&format!("**{}**, ", perm_name));
                                command_example.push_str(&format!(" {}", perm_name));
                            });

                            self.send_error(ctx.clone(), msg.clone(),
                                            &format!("**Missing permissions:**\n\
                                            This command requires this permissions: {}\n\
                                            Use this command to add needed permissions:\n\
                                            ```{}```",
                                                     needed_perms.trim_end_matches(", "),
                                                     command_example));
                            return;
                        }
                    }

                    if let Err(why) = c.exe(&ctx, &msg, guild.to_owned()) {
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