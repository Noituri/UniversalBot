// TODO: don't reduce warns if the length of user's warns is less than 1
use crate::command::{get_args, parse_args, ArgOption, Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR, EMBED_ERROR_COLOR};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::utils::db::{ServerInfo, create_action, ActionType, get_actions_by_kind, get_user_warn_lvl};
use crate::utils::object_finding::{get_member_from_id, FindObject};
use crate::bot_modules::main::help_command;
use crate::utils::special_entities_tools::send_to_mod_logs;
use crate::database::models::Action;

pub struct ModToolsCommand;

impl ModToolsCommand {
    fn reduce_warns(&self, ctx: &Context, msg: &Message, info: &ServerInfo) -> Result<(), String> {
        let member = match get_member_from_id(ctx, msg, get_args(msg.to_owned(), true), 1)? {
            Some(m) => m,
            None => return Ok(())
        };

        let lvl = get_user_warn_lvl(info, &member.get_id().to_string());
        if lvl < 1 {
            return Err("User already has the lowest possible warn level!".to_string())
        }

        create_action(
            info,
            msg.author.id.to_string(),
            Some(member.get_id().to_string()),
            ActionType::ReducedWarn,
            format!("Warn level reduced by **{}**", msg.author.name)
        );

        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Done!");
                e.description("Successfully reduced the warn level!");
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });

        Ok(())
    }

    fn gather_info<'a>(&self, title: &'a str, types: Vec<ActionType>, user_id: &str, fields: &mut Vec<(&'a str, String)>, info: &ServerInfo)
        -> Result<(), String> {
        match get_actions_by_kind(info, user_id.to_owned(), types) {
            Some(actions) => {
                if actions.len() == 0 {
                    return Err("This user has no reported mischiefs".to_string())
                }
                let mut report_msg = String::new();
                for (i, a) in actions.iter().enumerate() {
                    let mut reason = a.clone().message;
                    let temp_reasons: Vec<&str> = reason.split(". Reason:").collect();
                    if temp_reasons.len() > 1 {
                        reason = temp_reasons[1..].join(". ")
                    }
                    report_msg.push_str(&format!("**{}.** {}\n", actions.len() - i, reason))
                }
                fields.push((title, report_msg))
            },
            None => return Err("This user has no reported mischiefs".to_string())
        }
        Ok(())
    }

    fn show_report(&self, ctx: &Context, msg: &Message, args: Vec<String>, info: &ServerInfo) -> Result<(), String> {
        let member = match get_member_from_id(ctx, msg, get_args(msg.to_owned(), true), 1)? {
            Some(m) => m,
            None => return Ok(())
        };

        let user_id = &member.get_id().to_string();
        let report_message = format!("**Warns level:** {}\n", get_user_warn_lvl(info, &member.get_id().to_string()));
        let mut fields: Vec<(&str, String)> = Vec::new();
        if args.len() == 2 {
            match args[1].as_str() {
                "warns" => self.gather_info("Warns", vec![ActionType::Warn, ActionType::ReducedWarn], user_id, &mut fields, info)?,
                "bans" => self.gather_info("Bans", vec![ActionType::Ban, ActionType::UnBan], user_id, &mut fields, info)?,
                "mutes" => self.gather_info("Mutes", vec![ActionType::Mute, ActionType::UnMute], user_id, &mut fields, info)?,
                "kicks" => self.gather_info("Kicks", vec![ActionType::Kick], user_id, &mut fields, info)?,
                _ => return Err(format!("Type `{}` does not exist!", args[1]))
            }
        } else {
            self.gather_info("Warns", vec![ActionType::Warn, ActionType::ReducedWarn], user_id, &mut fields, info);
            self.gather_info("Bans", vec![ActionType::Ban, ActionType::UnBan], user_id, &mut fields, info);
            self.gather_info("Mutes", vec![ActionType::Mute, ActionType::UnMute], user_id, &mut fields, info);
            self.gather_info("Kicks", vec![ActionType::Kick], user_id, &mut fields, info);
        }

        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("Mod-Tools - {}", member.get_name()));
                e.description(&report_message);
                e.color(EMBED_REGULAR_COLOR);
                match member.user.read().avatar_url() {
                    Some(avatar) => e.thumbnail(avatar),
                    None => e
                };
                for f in fields {
                    e.field(f.0, f.1, true);
                }
                e
            });
            m
        });

        Ok(())
    }
}

impl Command for ModToolsCommand {
    fn name(&self) -> String {
        String::from("modtools")
    }

    fn desc(&self) -> String {
        String::from("Moderation tools. Check what kind of mischief someone did.")
    }

    fn use_in_dm(&self) -> bool {
        false
    }

    fn args(&self) -> Option<Vec<CommandArg>> {
        Some(vec![
            CommandArg {
                name: "<user>".to_string(),
                desc: Some("reduces warn level".to_string()),
                option: Some(ArgOption::User),
                next: Some(Box::new(CommandArg {
                    name: "reduce-warns".to_string(),
                    desc: None,
                    option: None,
                    next: None
                }))
            },
            CommandArg {
                name: "<user>".to_string(),
                desc: Some("shows report about a user".to_string()),
                option: Some(ArgOption::User),
                next: Some(Box::new(CommandArg {
                    name: "[warns/bans/mutes/kicks]".to_string(),
                    desc: None,
                    option: Some(ArgOption::Any),
                    next: None
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
        Some(vec!["modtools".to_string()])
    }

    fn config(&self) -> Option<Vec<CommandConfig>> {
        None
    }

    fn exe(&self, ctx: &Context, msg: &Message, info: &ServerInfo) -> Result<(), String> {
        let args = get_args(msg.clone(), false);
        match parse_args(&self.args().unwrap(), &args) {
            Ok(routes) => {
                match routes {
                    Some(path) => {
                        if path.len() == 2 && path[1].name == "reduce-warns" {
                            self.reduce_warns(ctx, msg, info)?;
                        } else {
                            self.show_report(ctx, msg, args, info)?;
                        }
                    },
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
