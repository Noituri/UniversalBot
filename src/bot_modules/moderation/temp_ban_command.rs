use crate::command::{
    get_args, parse_args, ArgOption, Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR,
};
use crate::database::get_db_con;
use crate::database::models::Role;
use crate::database::schema::roles;
use diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::database::schema::roles::columns::perms;
use crate::utils::db::{ServerInfo, get_db_role_by_id, create_action, ActionType, create_temp_ban_mute};
use crate::utils::object_finding::{get_role_from_id, get_member_from_id};
use crate::utils::perms::{get_module_perms, perms_exists};
use crate::bot_modules::main::help_command;
use crate::utils::{get_time_string, TimeFormat};

pub struct TempBanCommand;

impl TempBanCommand {
    fn ban(&self, ctx: &Context, msg: &Message, args: Vec<String>, info: &ServerInfo) -> Result<(), String> {
        let member = match get_member_from_id(ctx, msg, get_args(msg.to_owned(), true), 1)? {
            Some(m) => m,
            None => return Ok(())
        };

        if member.user_id() == ctx.cache.read().user.id {
            return Err("What did I do to you?".to_string())
        }

        // TODO check if mod-logs channel exist and send message there
        let reason = if args.len() > 2 {
            args[2..].join(" ")
        } else {
            String::new()
        };

        let reason_action_msg = if args.len() > 1 {
            format!(". Reason: {}.", reason)
        } else {
            "!".to_string()
        };

        let action_message = format!("User {} has been temp-banned for {}{}",
                                     member.display_name(),
                                     get_time_string(&args[1], TimeFormat::Hours),
                                     reason_action_msg);

        match member.ban(&ctx.http, &reason) {
            Ok(_) => create_action(
                info,
                msg.author.id.to_string(),
                Some(member.user_id().to_string()),
                ActionType::Ban,
                action_message.to_owned()
            ),
            Err(e) => return Err("Could not ban the user. Check permissions!".to_string())
        }

        // create_temp_ban_mute(info, member.user_id().to_string())

        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Ban - Done!");
                e.description(&action_message);
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });

        Ok(())
    }
}

impl Command for TempBanCommand {
    fn name(&self) -> String {
        String::from("tempBan")
    }

    fn desc(&self) -> String {
        String::from("Temporarily Banish users from your server.")
    }

    fn use_in_dm(&self) -> bool {
        false
    }

    fn args(&self) -> Option<Vec<CommandArg>> {
        Some(vec![
            CommandArg {
                name: "<user>".to_string(),
                desc: Some("bans user for given amount of time. `<time>` by default is in hours. You can change that by adding `m`\
                for minutes, `h` for hours, `d` for days behind the `<time>`. Example: `2d`.".to_string()),
                option: Some(ArgOption::User),
                next: Some(Box::new(CommandArg {
                    name: "<time>".to_string(),
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
                    Some(path) => self.ban(ctx, msg, args, info)?,
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
}
