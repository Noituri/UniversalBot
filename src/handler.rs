use crate::command::Command;
use crate::utils::{get_server, has_perms};
use log::{error, info};
use lazy_static::lazy_static;
use std::sync::Mutex;
use serenity::{
    model::{channel::Message, gateway::Ready, guild},
    prelude::*,
};

pub struct Handler;

#[derive(Clone)]
pub enum FindType {
    Role,
    User,
    Channel
}

#[derive(Clone)]
pub struct FindsAwaitingAnswer {
    pub find_type: FindType,
    pub who: u64,
    pub when: i32,
    pub finds: Vec<(u64, String)>,
    pub replace_text: String,
    pub msg_content: String
}

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
        let answer = if let Ok(num) = msg.content.trim().parse::<usize>() {
            num
        } else {
            return false
        };

        let mut picked = FindsAwaitingAnswer{
            find_type: FindType::Role,
            who: 0,
            when: 0,
            finds: vec![],
            replace_text: "".to_owned(),
            msg_content: "".to_string()
        };

        {
            let mut state = STATE.lock().unwrap();
            for (i, v) in state.role_finds_awaiting.iter().enumerate() {
                if v.who == msg.author.id.0 {
                    if answer < 0 || answer > v.finds.len() {
                        self.send_error(ctx, msg.to_owned(), "Your answer does not match any found roles!");
                        return true
                    }
                    picked = v.clone();
                    state.role_finds_awaiting.remove(i);
                    break;
                }
            }
        }

        if picked.who == 0 {
            return false
        }

        msg.content = picked.msg_content.replacen(
            &picked.replace_text,
            &picked.finds[answer-1].0.to_string(),
            1
        );
        false
    }
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, mut msg: Message) {
        // ignore other bots
        if msg.author.bot {
            return
        }

        if self.check_awaiting_answers(ctx.to_owned(), &mut msg) {
            return
        }

        let guild = get_server(msg.guild_id);
        let prefix = if msg
            .content
            .starts_with(&format!("<@{}> ", ctx.cache.read().user.id))
        {
            format!("<@{}> ", ctx.cache.read().user.id)
        } else if msg
            .content
            .starts_with(&format!("<@!{}> ", ctx.cache.read().user.id))
        {
            format!("<@!{}> ", ctx.cache.read().user.id)
        } else if let Some(g) = guild.to_owned() {
            g.prefix
        } else {
            super::config::DEFAULT_PREFIX.to_owned()
        };

        if msg.content.trim() == format!("<@{}>", ctx.cache.read().user.id)
            || msg.content.trim() == format!("<@!{}>", ctx.cache.read().user.id)
        {
            let _ = super::bot_modules::main::help_command::HelpCommand {}.exe(&ctx, &msg, guild);
            return
        }

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
                        self.send_error(
                            ctx.clone(),
                            msg.clone(),
                            "This command is disabled in DM chat!",
                        );
                        return;
                    } else if !msg.is_private() {
                        if !has_perms(&ctx, &msg, guild.clone().unwrap(), &c.perms()) {
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
