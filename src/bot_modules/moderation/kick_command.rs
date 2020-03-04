use crate::command::{
    get_args, parse_args, ArgOption, Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR,
};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::utils::db::{ServerInfo, create_action, ActionType};
use crate::bot_modules::main::help_command;
use crate::utils::object_finding::{get_member_from_id, FindObject};
use crate::utils::special_entities_tools::send_to_mod_logs;

pub struct KickCommand;

impl KickCommand {
    fn kick(&self, ctx: &Context, msg: &Message, args: Vec<String>, info: &ServerInfo) -> Result<(), String> {
        let member = match get_member_from_id(ctx, msg, get_args(msg.to_owned(), true), 1)? {
            Some(m) => m,
            None => return Ok(())
        };

        if member.user_id() == ctx.cache.read().user.id {
            return Err("Why me?".to_string())
        }

        let action_msg = if args.len() > 1 {
            format!("User has been kicked out! Reason {}.", args[1..].join(" "))
        } else {
            String::from("User has been kicked out!")
        };

        match &ctx.http.kick_member(msg.guild_id.unwrap().0, member.get_id()) {
            Ok(_) => create_action(
                info,
                msg.author.id.to_string(),
                Some(member.get_id().to_string()),
                ActionType::Kick,
                action_msg.to_owned()
            ),
            Err(_) => return Err("Could not kick the user. Check permissions!".to_string())
        }

        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Kick - Done!");
                e.description(&action_msg);
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });

        send_to_mod_logs(ctx, info, "Kick", &action_msg);
        Ok(())
    }
}

impl Command for KickCommand {
    fn name(&self) -> String {
        String::from("kick")
    }

    fn desc(&self) -> String {
        String::from("Kicks user from your server.")
    }

    fn use_in_dm(&self) -> bool {
        false
    }

    fn args(&self) -> Option<Vec<CommandArg>> {
        Some(vec![
            CommandArg {
                name: "<user>".to_string(),
                desc: Some("kicks the user".to_string()),
                option: Some(ArgOption::User),
                next: Some(Box::new(CommandArg{
                    name: "[reason...]".to_string(),
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
        Some(vec!["kick".to_string()])
    }

    fn config(&self) -> Option<Vec<CommandConfig>> {
        None
    }

    fn exe(&self, ctx: &Context, msg: &Message, info: &ServerInfo) -> Result<(), String> {
        let args = get_args(msg.clone(), false);
        match parse_args(&self.args().unwrap(), &args) {
            Ok(routes) => {
                match routes {
                    Some(_) => self.kick(ctx, msg, args, info)?,
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