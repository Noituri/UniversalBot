use crate::command::{
    get_args, parse_args, ArgOption, Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR,
};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::utils::db::{ServerInfo, create_action, ActionType, create_temp_ban_mute};
use crate::utils::object_finding::get_member_from_id;
use crate::bot_modules::main::help_command;
use crate::utils::get_time;
use std::thread;
use crate::database::get_db_con;
use std::sync::Mutex;
use std::time::Duration;
use crate::database::models::{Server, TempBanMute};
use crate::database::schema::{servers, temp_bans_mutes};
use crate::database::schema::temp_bans_mutes::columns::{id, action_type};
use crate::diesel::{RunQueryDsl, BelongingToDsl, ExpressionMethods, QueryDsl, GroupedBy};
use chrono::Utc;
use log::error;

pub struct MuteCommand;

impl MuteCommand {
    fn ban(&self, ctx: &Context, msg: &Message, path: Vec<CommandArg>, args: Vec<String>, info: &ServerInfo) -> Result<(), String> {
        let member = match get_member_from_id(ctx, msg, get_args(msg.to_owned(), true), 1)? {
            Some(m) => m,
            None => return Ok(())
        };

        if member.user_id() == ctx.cache.read().user.id {
            return Err("I thought we were friends!".to_string())
        }

        // TODO check if mod-logs channel exist and send message there


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
    //     let ctx = Mutex::new(ctx.clone());
    //     thread::spawn(move || {
    //         let db = get_db_con().get().expect("Could not get db pool!");
    //         loop {
    //             thread::sleep(Duration::from_secs(5));
    //             let servers = servers::dsl::servers
    //                 .load::<Server>(&db)
    //                 .expect("Could not load servers!");
    //
    //             let unbans = TempBanMute::belonging_to(&servers)
    //                 .filter(action_type.eq(ActionType::Ban as i32))
    //                 .load::<TempBanMute>(&db)
    //                 .expect("Could not load temp bans and mutes")
    //                 .grouped_by(&servers);
    //
    //             let data = servers.into_iter().zip(unbans).collect::<Vec<_>>();
    //
    //             for v in data {
    //                 for ub in v.1 {
    //                     if ub.end_date < Utc::now().naive_utc() {
    //                         let guild_id = v.0.guildid.parse::<u64>().unwrap();
    //                         let user_id = ub.user_id.parse::<u64>().unwrap();
    //                         match &ctx.lock().unwrap().http.remove_ban(guild_id, user_id) {
    //                             Ok(_) => {/* send dm */},
    //                             Err(_) => error!("Could not unban user")
    //                         }
    //                         let _ = diesel::delete(temp_bans_mutes::table.filter(id.eq(ub.id)))
    //                             .execute(&db);
    //                     }
    //                 }
    //             }
    //         }
    //     });
    }
}
