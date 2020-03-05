use crate::command::{
    get_args, parse_args, ArgOption, Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR,
};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::utils::db::{ServerInfo, create_action, ActionType, create_temp_ban_mute, get_special_entity_by_type};
use crate::utils::object_finding::get_member_from_id;
use crate::bot_modules::main::help_command;
use crate::utils::get_time;
use std::thread;
use crate::database::get_db_con;
use std::sync::Mutex;
use std::time::Duration;
use crate::database::models::{Server, TempBanMute, SpecialEntityType, SpecialEntity};
use crate::database::schema::{servers, temp_bans_mutes};
use crate::database::schema::temp_bans_mutes::columns::{id, action_type};
use crate::diesel::{RunQueryDsl, BelongingToDsl, ExpressionMethods, QueryDsl, GroupedBy};
use chrono::Utc;
use log::error;
use crate::utils::special_entities_tools::send_to_mod_logs;
use crate::config::DEFAULT_PREFIX;
use crate::database::schema::special_entities::columns::entity_type;

pub struct MuteCommand;

impl MuteCommand {
    fn mute(&self, ctx: &Context, msg: &Message, path: Vec<CommandArg>, args: Vec<String>, info: &ServerInfo) -> Result<(), String> {
        let prefix = if let Some(s) = info.server.clone() {
            s.prefix
        } else {
            DEFAULT_PREFIX.to_string()
        };

        let mute_role_id = match get_special_entity_by_type(info, SpecialEntityType::MuteRole) {
            Some(r) => r.entity_id,
            None => return Err(format!("There is no muted role. Please use `{}setup muted-role`!", prefix))
        };

        let mut member = match get_member_from_id(ctx, msg, get_args(msg.to_owned(), true), 1)? {
            Some(m) => m,
            None => return Ok(())
        };

        if member.user_id() == ctx.cache.read().user.id {
            return Err("I thought we were friends!".to_string())
        }

        if member.user_id() == msg.author.id {
            return Err("Let's keep talking!".to_string())
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
            format!("User {} has been temp-muted for {}{}",
                    member.display_name(),
                    &args[1],
                    reason_action_msg
            )
        } else {
            format!("User {} has been muted{}", member.display_name(), reason_action_msg)
        };

        match member.add_role(&ctx.http, mute_role_id.parse::<u64>().unwrap()) {
            Ok(_) => create_action(
                info,
                msg.author.id.to_string(),
                Some(member.user_id().to_string()),
                ActionType::Mute,
                action_message.to_owned()
            ),
            Err(_) => return Err("Could not mute the user. Check permissions!".to_string())
        }

        if is_temp {
            create_temp_ban_mute(
                info,
                member.user_id().to_string(),
                get_time(&args[1])?,
                ActionType::Mute
            );
        }

        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Mute - Done!");
                e.description(&action_message);
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });

        send_to_mod_logs(ctx, info, "Mute", &action_message);
        Ok(())
    }
}

impl Command for MuteCommand {
    fn name(&self) -> String {
        String::from("mute")
    }

    fn desc(&self) -> String {
        String::from("Mute system.")
    }

    fn use_in_dm(&self) -> bool {
        false
    }

    fn args(&self) -> Option<Vec<CommandArg>> {
        Some(vec![
            CommandArg {
                name: "<user>".to_string(),
                desc: Some("mutes user. If `[time]` is provided then user will be temp-muted. \
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
        Some(vec!["mute".to_string()])
    }

    fn config(&self) -> Option<Vec<CommandConfig>> {
        None
    }

    fn exe(&self, ctx: &Context, msg: &Message, info: &ServerInfo) -> Result<(), String> {
        let args = get_args(msg.clone(), false);
        match parse_args(&self.args().unwrap(), &args) {
            Ok(routes) => {
                match routes {
                    Some(path) => self.mute(ctx, msg, path, args, info)?,
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
                thread::sleep(Duration::from_secs(5));
                let servers = servers::dsl::servers
                    .load::<Server>(&db)
                    .expect("Could not load servers!");

                let unmutes = TempBanMute::belonging_to(&servers)
                    .filter(action_type.eq(ActionType::Mute as i32))
                    .load::<TempBanMute>(&db)
                    .expect("Could not load temp bans and mutes")
                    .grouped_by(&servers);

                let muted_roles: Vec<SpecialEntity> = SpecialEntity::belonging_to(&servers)
                    .filter(entity_type.eq(SpecialEntityType::MuteRole as i32))
                    .load::<SpecialEntity>(&db)
                    .expect("Could not load temp bans and mutes");

                let data = servers.into_iter().zip(unmutes).collect::<Vec<_>>();

                for v in data {
                    for m in v.1.iter() {
                        if m.end_date < Utc::now().naive_utc() {
                            let guild_id = v.0.guildid.parse::<u64>().unwrap();
                            let user_id = m.user_id.parse::<u64>().unwrap();
                            let role_id = muted_roles.iter().find(|r| r.server_id == v.0.id);
                            match role_id {
                                Some (r) => {
                                    let r_id = r.entity_id.parse::<u64>().unwrap();
                                    match &ctx.lock().unwrap().http.remove_member_role(guild_id, user_id, r_id) {
                                        Ok(_) => {/* send dm */},
                                        Err(_) => error!("Could not unmute user")
                                    }
                                }
                                None => {}
                            }
                            let _ = diesel::delete(temp_bans_mutes::table.filter(id.eq(m.id)))
                                .execute(&db);
                        }
                    }
                }
            }
        });
    }
}
