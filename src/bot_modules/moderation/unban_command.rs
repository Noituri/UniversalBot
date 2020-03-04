use crate::command::{
    get_args, parse_args, ArgOption, Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR,
};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::utils::db::{ServerInfo, create_action, ActionType};
use crate::bot_modules::main::help_command;
use crate::utils::special_entities_tools::send_to_mod_logs;

pub struct UnBanCommand;

impl UnBanCommand {
    fn unban(&self, ctx: &Context, msg: &Message, args: Vec<String>, info: &ServerInfo) -> Result<(), String> {
        let user_id = if msg.mentions.len() != 0 {
            msg.mentions[0].id.0
        } else {
            args[0].parse::<u64>().unwrap()
        };

        if user_id == ctx.cache.read().user.id.0 {
            return Err("Hmmm?".to_string())
        }

        let user = match &ctx.http.get_user(user_id) {
            Ok(u) => u.clone(),
            Err(_) => return Err("User does not exist".to_string())
        };

        let action_message = format!("User {} has been unbanned!", user.name);
        match &ctx.http.remove_ban(msg.guild_id.unwrap().0, user_id) {
            Ok(_) => create_action(
                info,
                msg.author.id.to_string(),
                Some(user_id.to_string()),
                ActionType::UnBan,
                action_message.to_owned()
            ),
            Err(_) => return Err("Could not unban the user. Check permissions!".to_string())
        }

        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Unban - Done!");
                e.description(&action_message);
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });

        send_to_mod_logs(ctx, info, "Unban", &action_message);

        Ok(())
    }
}

impl Command for UnBanCommand {
    fn name(&self) -> String {
        String::from("unban")
    }

    fn desc(&self) -> String {
        String::from("Unbans user from your server.")
    }

    fn use_in_dm(&self) -> bool {
        false
    }

    fn args(&self) -> Option<Vec<CommandArg>> {
        Some(vec![
            CommandArg {
                name: "<userID>".to_string(),
                desc: Some("unbans user".to_string()),
                option: Some(ArgOption::UserId),
                next: None
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
                    Some(_) => self.unban(ctx, msg, args, info)?,
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