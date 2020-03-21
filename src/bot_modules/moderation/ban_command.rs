use crate::command::{
    get_args, parse_args, ArgOption, Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR,
};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::utils::db::{ServerInfo, create_action, ActionType, create_temp_operation};
use crate::utils::object_finding::get_member_from_id;
use crate::bot_modules::main::help_command;
use crate::utils::get_time;
use std::thread;
use crate::database::get_db_con;
use std::sync::Mutex;
use std::time::Duration;
use crate::database::models::{Server, TempOperation};
use crate::database::schema::{servers, temp_operations};
use crate::database::schema::temp_operations::columns::{id, action_type};
use crate::diesel::{RunQueryDsl, BelongingToDsl, ExpressionMethods, QueryDsl, GroupedBy};
use chrono::Utc;
use log::error;
use crate::utils::special_entities_tools::send_to_mod_logs;

pub struct BanCommand;

impl BanCommand {
    fn ban(&self, ctx: &Context, msg: &Message, path: Vec<CommandArg>, args: Vec<String>, info: &ServerInfo) -> Result<(), String> {
        let member = match get_member_from_id(ctx, msg, get_args(msg.to_owned(), true), 1)? {
            Some(m) => m,
            None => return Ok(())
        };

        if member.user_id() == ctx.cache.read().user.id {
            return Err("What did I do to you?".to_string())
        }
        if member.user_id() == msg.author.id {
            return Err("I think not".to_string())
        }

        let mut reason = String::new();
        let mut is_temp = false;

        if path.len() > 1 {
            if path[1].name == "[time]" {
                is_temp = true;
                if path.len() > 2 {
                    reason = args[2..].join(" ");
                }
            } else {
                reason = args[1..].join(" ");
            }
        }


        let reason_action_msg = if !reason.is_empty() {
            format!(". Reason: {}.", reason)
        } else {
            "!".to_string()
        };

        let action_message = if is_temp {
            format!("User {} has been temp-banned for {}{}",
                    member.display_name(),
                    &args[1],
                    reason_action_msg
            )
        } else {
            format!("User {} has been banned{}", member.display_name(), reason_action_msg)
        };

        match member.ban(&ctx.http, &reason) {
            Ok(_) => create_action(
                info,
                msg.author.id.to_string(),
                Some(member.user_id().to_string()),
                ActionType::Ban,
                action_message.to_owned()
            ),
            Err(_) => return Err("Could not ban the user. Check permissions!".to_string())
        }

        if is_temp {
            create_temp_operation(
                info,
                member.user_id().to_string(),
                get_time(&args[1])?,
                ActionType::Ban
            );
        }

        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Ban - Done!");
                e.description(&action_message);
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });

        send_to_mod_logs(ctx, info, "Ban", &action_message);

        Ok(())
    }
}

impl Command for BanCommand {
    fn name(&self) -> String {
        String::from("ban")
    }

    fn desc(&self) -> String {
        String::from("Banish users from your server.")
    }

    fn use_in_dm(&self) -> bool {
        false
    }

    fn args(&self) -> Option<Vec<CommandArg>> {
        Some(vec![
            CommandArg {
                name: "<user>".to_string(),
                desc: Some("bans user. If `[time]` is provided then user will be temp-banned. \
                You create `[time]` by adding to desired time: `m` \
                for minutes, `h` for hours, `d` for days behind the `time`. \nExample: `2d`.".to_string()),
                option: Some(ArgOption::User),
                next: Some(Box::new(CommandArg {
                    name: "[time]".to_string(),
                    desc: None,
                    option: Some(ArgOption::Time),
                    next: Some(Box::new(CommandArg {
                        name: "[reason...]".to_string(),
                        desc: None,
                        option: Some(ArgOption::Any),
                        next: None,
                    })),
                }))
            },
            CommandArg {
                name: "".to_string(),
                desc: Some("shows usage information".to_string()),
                option: None,
                next: None
            }
        ])
    }

    fn perms(&self) -> Option<Vec<String>> {
        Some(vec!["ban".to_string()])
    }

    fn config(&self) -> Option<Vec<CommandConfig>> {
        None
    }

    fn exe(&self, ctx: &Context, msg: &Message, info: &ServerInfo) -> Result<(), String> {
        let args = get_args(msg.clone(), false);
        match parse_args(&self.args().unwrap(), &args) {
            Ok(routes) => {
                match routes {
                    Some(path) => self.ban(ctx, msg, path, args, info)?,
                    None => {
                        let help_cmd = help_command::HelpCommand {};
                        help_cmd.show_cmd_details(ctx, msg, info, self.name())?;
                    }
                }
            }
            Err(why) => return Err(why),
        }
        Ok(())
    }

    fn init(&self, ctx: &Context) {
        let ctx = Mutex::new(ctx.clone());
        thread::spawn(move || {
            let db = get_db_con().get().expect("Could not get db pool!");
            loop {
                thread::sleep(Duration::from_secs(20));
                let servers = servers::dsl::servers
                    .load::<Server>(&db)
                    .expect("Could not load servers!");

                let unbans = TempOperation::belonging_to(&servers)
                    .filter(action_type.eq(ActionType::Ban as i32))
                    .load::<TempOperation>(&db)
                    .expect("Could not load temp operations")
                    .grouped_by(&servers);

                let data = servers.into_iter().zip(unbans).collect::<Vec<_>>();

                for v in data {
                    for ub in v.1 {
                        if ub.end_date < Utc::now().naive_utc() {
                            let guild_id = v.0.guildid.parse::<u64>().unwrap();
                            let user_id = ub.target_id.parse::<u64>().unwrap();
                            match &ctx.lock().unwrap().http.remove_ban(guild_id, user_id) {
                                Ok(_) => {/* send dm */},
                                Err(_) => error!("Could not unban user")
                            }
                            let _ = diesel::delete(temp_operations::table.filter(id.eq(ub.id)))
                                .execute(&db);
                        }
                    }
                }
            }
        });
    }
}
