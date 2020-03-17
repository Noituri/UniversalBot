use crate::command::Command;
use log::{error, info};
use lazy_static::lazy_static;
use std::sync::Mutex;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use chrono::{Utc, Duration};
use crate::utils::object_finding::FindsAwaitingAnswer;
use crate::utils::perms::has_perms;
use crate::utils::db::ServerInfo;
use crate::bot_modules::get_modules;

pub struct Handler;

#[derive(Default)]
pub struct State {
    pub role_finds_awaiting: Vec<FindsAwaitingAnswer>,
}

lazy_static! {
    pub static ref STATE: Mutex<State> = Mutex::new(State::default());
}

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

    fn check_awaiting_answers(&self, ctx: Context, msg: &mut Message) -> bool {
        {
            let mut state = STATE.lock().unwrap();

            for i in (0..state.role_finds_awaiting.len()).rev() {
                let tmp = &state.role_finds_awaiting[i];
                if tmp.when + Duration::seconds(30) < Utc::now() {
                    state.role_finds_awaiting.remove(i);
                }
            }
        }

        let answer = if let Ok(num) = msg.content.trim().parse::<usize>() {
            num
        } else {
            return false;
        };

        let mut picked = FindsAwaitingAnswer {
            who: 0,
            channel: 0,
            when: Utc::now(),
            finds: vec![],
            args: vec![],
            replace_index: 0
        };

        {
            let mut state = STATE.lock().unwrap();
            for (i, v) in state.role_finds_awaiting.iter().enumerate() {
                if v.who == msg.author.id.0 && v.channel == msg.channel_id.0 {
                    if answer > v.finds.len() {
                        self.send_error(ctx, msg.to_owned(), "Your answer does not match any found options!");
                        return true;
                    }

                    picked = v.clone();
                    state.role_finds_awaiting.remove(i);
                    break;
                }
            }
        }

        if picked.who == 0 {
            return false;
        }

        picked.args[picked.replace_index] = picked.finds[answer-1].0.to_string();
        msg.content = picked.args.join(" ");

        false
    }
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, mut msg: Message) {
        // ignore other bots
        if msg.author.bot {
            return;
        }

        if self.check_awaiting_answers(ctx.to_owned(), &mut msg) {
            return;
        }

        let info = ServerInfo::new(msg.guild_id);
        let prefix = if msg.content.starts_with(&format!("<@{}> ", ctx.cache.read().user.id)) {
            format!("<@{}> ", ctx.cache.read().user.id)
        } else if msg.content.starts_with(&format!("<@!{}> ", ctx.cache.read().user.id)) {
            format!("<@!{}> ", ctx.cache.read().user.id)
        } else if let Some(g) = info.server.to_owned() {
            g.prefix
        } else {
            super::config::DEFAULT_PREFIX.to_owned()
        };

        if msg.content.trim() == format!("<@{}>", ctx.cache.read().user.id)
            || msg.content.trim() == format!("<@!{}>", ctx.cache.read().user.id)
        {
            let _ = super::bot_modules::main::help_command::HelpCommand {}.exe(&ctx, &msg, &info);
            return;
        }

        let check_msg = msg.content.to_lowercase();
        for m in super::get_modules().iter() {
            if !m.enabled(&info) {
                continue
            }

            for c in m.commands().iter() {
                if !c.enabled(&info) {
                    continue;
                }

                if check_msg.starts_with(&format!("{}{}", prefix, c.name().to_lowercase())) {
                    if !c.use_in_dm() && msg.is_private() {
                        self.send_error(
                            ctx.clone(),
                            msg.clone(),
                            "This command is disabled in DM chat!",
                        );
                        return;
                    } else if !msg.is_private() {
                        if !has_perms(&ctx, &msg, &info, &c.perms()) {
                            let mut needed_perms = String::new();
                            let mut command_example =
                                format!("{}perms add <@role/role_name/role_id>", prefix);
                            c.perms().unwrap().iter().for_each(|p| {
                                let perm_name = p.to_uppercase();
                                needed_perms.push_str(&format!("**{}**, ", perm_name));
                                command_example.push_str(&format!(" {}", perm_name));
                            });

                            self.send_error(
                                ctx.clone(),
                                msg.clone(),
                                &format!(
                                    "**Missing permissions:**\n\
                                     This command requires this permissions: {}\n\
                                     Use this command to add needed permissions:\n\
                                     ```{}```",
                                    needed_perms.trim_end_matches(", "),
                                    command_example
                                ),
                            );
                            return;
                        }
                    }

                    if let Err(why) = c.exe(&ctx, &msg, &info) {
                        error!("Command '{}' failed. Reason: {}", c.name(), why.to_owned());
                        self.send_error(ctx.clone(), msg.clone(), &why);
                    }

                    break;
                }
            }
        }
    }

    fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
        for m in get_modules() {
            for c in m.commands() {
                c.init(&ctx);
                info!("Command {} initialized!", c.name())
            }
        }
    }
}
