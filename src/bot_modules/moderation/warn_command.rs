use crate::command::{get_args, parse_args, ArgOption, Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR, EMBED_ERROR_COLOR};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::utils::db::{ServerInfo, create_action, ActionType};
use crate::utils::object_finding::{get_member_from_id, FindObject};
use crate::bot_modules::main::help_command;
use crate::utils::special_entities_tools::send_to_mod_logs;

pub struct WarnCommand;

impl WarnCommand {
    fn warn(&self, ctx: &Context, msg: &Message, args: Vec<String>, info: &ServerInfo) -> Result<(), String> {
        let member = match get_member_from_id(ctx, msg, get_args(msg.to_owned(), true), 1)? {
            Some(m) => m,
            None => return Ok(())
        };

        if member.user_id() == ctx.cache.read().user.id {
            return Err("I did not do anything wrong sir. Did I?".to_string())
        }
        if member.user_id() == msg.author.id {
            return Err(r#"¯\_(ツ)_/¯"#.to_string())
        }

        let reason = args[1..].join(" ");
        let action_message = format!("User {} has been warned. Reason: {}!", member.display_name(), reason);
        let _ = member.user.read().direct_message(ctx.clone().http, |m| {
            m.embed(|e| {
                e.title("Warn!");
                e.description(format!("You have been warned! Reason: {}", reason));
                e.color(EMBED_ERROR_COLOR);
                e
            });
            m
        });

        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Warned!");
                e.description(&action_message);
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });

        create_action(
            info,
            msg.author.id.to_string(),
            Some(member.get_id().to_string()),
            ActionType::Warn,
            action_message.to_owned()
        );
        send_to_mod_logs(ctx, info, "Warn", &action_message);
        Ok(())
    }
}

impl Command for WarnCommand {
    fn name(&self) -> String {
        String::from("warn")
    }

    fn desc(&self) -> String {
        String::from("Warn system.")
    }

    fn use_in_dm(&self) -> bool {
        false
    }

    fn args(&self) -> Option<Vec<CommandArg>> {
        Some(vec![
            CommandArg {
                name: "<user>".to_string(),
                desc: Some("warns user".to_string()),
                option: Some(ArgOption::User),
                next: Some(Box::new(CommandArg{
                    name: "<reason...>".to_string(),
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
        Some(vec!["warn".to_string()])
    }

    fn config(&self) -> Option<Vec<CommandConfig>> {
        None
    }

    fn exe(&self, ctx: &Context, msg: &Message, info: &ServerInfo) -> Result<(), String> {
        let args = get_args(msg.clone(), false);
        match parse_args(&self.args().unwrap(), &args) {
            Ok(routes) => {
                match routes {
                    Some(_) => self.warn(ctx, msg, args, info)?,
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
