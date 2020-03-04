use crate::command::{get_args, parse_args, ArgOption, Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR, find_command, is_command_protected};
use crate::database::get_db_con;
use crate::database::models::*;
use crate::database::schema::commands::enabled_channels;
use crate::database::schema::*;
use diesel::prelude::*;
use serenity::model::channel::{Message, ChannelType};
use serenity::prelude::Context;
use crate::utils::db::{ServerInfo, get_db_command_by_name};
use crate::utils::object_finding::get_channel_from_id;

pub struct SetupCommand;

impl SetupCommand {

}

impl Command for SetupCommand {
    fn name(&self) -> String {
        String::from("setup")
    }

    fn desc(&self) -> String {
        String::from("Setup tool.")
    }

    fn use_in_dm(&self) -> bool {
        false
    }

    fn args(&self) -> Option<Vec<CommandArg>> {
        Some(vec![
            CommandArg {
                name: String::from("mute-role"),
                desc: Some(String::from("creates role used for mute command.")),
                option: Some(ArgOption::Any),
                next: Some(Box::new(CommandArg {
                    name: String::from("[name]"),
                    desc: None,
                    option: Some(ArgOption::Any),
                    next: None,
                })),
            },
            CommandArg {
                name: String::from("modlogs-channel"),
                desc: Some(String::from("creates channel used for moderation logging.")),
                option: Some(ArgOption::Any),
                next: Some(Box::new(CommandArg {
                    name: String::from("[name]"),
                    desc: None,
                    option: Some(ArgOption::Any),
                    next: None,
                })),
            },
            CommandArg {
                name: String::from(""),
                desc: Some(String::from("shows usage information.")),
                option: None,
                next: None,
            },
        ])
    }

    fn perms(&self) -> Option<Vec<String>> {
        Some(vec!["setup".to_string()])
    }

    fn config(&self) -> Option<Vec<CommandConfig>> {
        None
    }

    fn exe(&self, ctx: &Context, msg: &Message, info: &ServerInfo) -> Result<(), String> {
        let args = get_args(msg.clone(), false);
        match parse_args(&self.args().unwrap(), &args) {
            Ok(routes) => match routes {
                Some(path) => {
                    Ok(())
                }
                None => {
                    let help_cmd = super::help_command::HelpCommand{};
                    help_cmd.show_cmd_details(ctx, msg, info, self.name())
                },
            },
            Err(why) => return Err(why),
        }
    }
}

