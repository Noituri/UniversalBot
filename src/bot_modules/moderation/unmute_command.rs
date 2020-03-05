use crate::command::{
    get_args, parse_args, ArgOption, Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR,
};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::utils::db::{ServerInfo, create_action, ActionType, get_special_entity_by_type};
use crate::utils::object_finding::get_member_from_id;
use crate::bot_modules::main::help_command;
use crate::database::models::SpecialEntityType;
use crate::utils::special_entities_tools::send_to_mod_logs;
use crate::config::DEFAULT_PREFIX;

pub struct UnMuteCommand;

impl UnMuteCommand {
    fn unmute(&self, ctx: &Context, msg: &Message, info: &ServerInfo) -> Result<(), String> {
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
            return Err("Whaa?".to_string())
        }

        let action_message = format!("User {} has been un-muted!", member.display_name());

        match member.remove_role(&ctx.http, mute_role_id.parse::<u64>().unwrap()) {
            Ok(_) => create_action(
                info,
                msg.author.id.to_string(),
                Some(member.user_id().to_string()),
                ActionType::UnMute,
                action_message.to_owned()
            ),
            Err(_) => return Err("Could not un-mute the user. Check permissions!".to_string())
        }

        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Un-Mute - Done!");
                e.description(&action_message);
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });

        send_to_mod_logs(ctx, info, "Un-Mute", &action_message);
        Ok(())
    }
}

impl Command for UnMuteCommand {
    fn name(&self) -> String {
        String::from("unmute")
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
                desc: Some("unmutes user".to_string()),
                option: Some(ArgOption::User),
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
                    Some(_) => self.unmute(ctx, msg, info)?,
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
