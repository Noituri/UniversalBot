use crate::command::Command;
use log::{error, info};
use lazy_static::lazy_static;
use std::sync::Mutex;
use serenity::model::channel::{Message, Reaction, ReactionType, PermissionOverwrite, PermissionOverwriteType};
use serenity::{
    model::gateway::Ready,
    model::id::ChannelId,
    model::Permissions,
    prelude::*,
};
use chrono::{Utc, Duration};
use crate::utils::object_finding::FindsAwaitingAnswer;
use crate::utils::perms::has_perms;
use crate::utils::db::{ServerInfo, ActionType};
use crate::bot_modules::get_modules;
use super::bot_modules::main::help_command::HelpCommand;
use super::bot_modules::tickets::solved_command::SolvedTicketCommand;
use crate::database::schema::{servers, temp_operations};
use crate::database::schema::temp_operations::columns::{id, action_type, target_id};
use crate::diesel::{RunQueryDsl, BelongingToDsl, ExpressionMethods, QueryDsl, BoolExpressionMethods, TextExpressionMethods};
use crate::database::get_db_con;

pub struct Handler;

#[derive(Default)]
pub struct State {
    pub role_finds_awaiting: Vec<FindsAwaitingAnswer>,
}

lazy_static! {
    pub static ref STATE: Mutex<State> = Mutex::new(State::default());
}

impl Handler {
    fn send_error(&self, ctx: Context, channel_id: ChannelId, why: &str) {
        let _ = channel_id.send_message(ctx.http, |m| {
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
                        self.send_error(ctx, msg.channel_id, "Your answer does not match any found options!");
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
            let _ = HelpCommand {}.exe(&ctx, &msg, &info);
            return;
        }

        let check_msg = msg.content.to_lowercase();
        for m in super::get_modules().iter() {
            if !m.enabled(&info) {
                continue
            }

            for c in m.commands().iter() {
                if c.disabled(&info, msg.channel_id.to_string()) {
                    continue;
                }

                if check_msg.starts_with(&format!("{}{}", prefix, c.name().to_lowercase())) {
                    if !c.use_in_dm() && msg.is_private() {
                        self.send_error(
                            ctx.clone(),
                            msg.channel_id,
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
                                msg.channel_id,
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
                        self.send_error(ctx.clone(), msg.channel_id, &why);
                    }

                    break;
                }
            }
        }
    }

    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if ctx.cache.read().user.id == reaction.user_id {
            return
        }

        if let ReactionType::Unicode(emoji) = &reaction.emoji {
            match emoji.as_str() {
                "✅" => {
                    match reaction.guild_id {
                        Some(guild) => {
                            let user = match reaction.user(ctx.http.clone()) {
                                Ok(u) => u,
                                Err(_) => return
                            };
                            let cmd = SolvedTicketCommand{};
                            if let Err(why) = cmd.solve(&ctx, reaction.channel_id, &user, &ServerInfo::new(Some(guild))) {
                                error!("Reaction Callback '✅' failed. Reason: {}", why.to_owned());
                            } else {
                                let _ = reaction.delete(ctx.http);
                            }
                        },
                        None => {}
                    }
                },
                "❎" => {
                    match reaction.guild_id {
                        Some(guild) => {
                            let result = diesel::delete(temp_operations::table.filter(
                                action_type.eq(ActionType::SolvedTicket as i32)
                                .and(target_id.like(reaction.channel_id.to_string()))
                            )).execute(&get_db_con().get().expect("Could not get db pool"));
                            match result {
                                Ok(removed) => {
                                    if removed == 0 {
                                        return
                                    }
                                },
                                Err(_) => return
                            }

                            let mut perms = Permissions::READ_MESSAGES;
                            perms.insert(Permissions::SEND_MESSAGES);
                            perms.insert(Permissions::ADD_REACTIONS);

                            let _ = reaction.channel_id.create_permission(&ctx.http, &PermissionOverwrite {
                                allow: perms,
                                deny: Permissions::empty(),
                                kind: PermissionOverwriteType::Member(reaction.user_id)
                            });

                            let _ = ctx.http.delete_message(reaction.channel_id.into(), reaction.message_id.into());
                        },
                        None => {}
                    }
                }
                _ => {}
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
