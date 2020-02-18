use crate::command::{get_args, parse_args, ArgOption, Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR, find_command, is_command_protected};
use crate::database::get_db_con;
use crate::database::models::*;
use crate::database::schema::commands::enabled_channels;
use crate::database::schema::*;
use diesel::prelude::*;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::utils::db::{ServerInfo, get_db_command_by_name};
use crate::utils::object_finding::get_channel_from_id;

pub struct CmdCommand;

impl CmdCommand {
   fn change_command(&self, ctx: &Context, msg: &Message, args: Vec<String>, info: &ServerInfo, is_channel: bool) -> Result<(), String> {
       find_command(&args[0], info)?;
       if is_command_protected(&args[0])? {
           return Err("Command is protected. It can't be modified!".to_string())
       }

       // TODO if `all` is used then add every channel
       let channel = if is_channel {
           match get_channel_from_id(ctx, msg, get_args(msg.to_owned(), true), 3)? {
               Some(ch) => ch.id.0.to_string(),
               None => return Ok(())
           }
       } else {
           msg.channel_id.0.to_string()
       };

       let mut cmd = match get_db_command_by_name(info, args[0].to_string()) {
           Some(c) => c,
           None => return Err("Could not find command in the database!".to_string())
       };

       if args[1] == "enable" && !cmd.enabled_channels.contains(&channel) {
           cmd.enabled_channels.push(channel)
       } else if args[1] == "disable" {
           for (i, c) in cmd.enabled_channels.iter().enumerate() {
               if c == &channel {
                   cmd.enabled_channels.remove(i);
                   break;
               }
           }
       }

       diesel::update(commands::dsl::commands.find(cmd.id))
           .set(enabled_channels.eq(cmd.enabled_channels))
           .get_result::<DBCommand>(&get_db_con().get().expect("Could not get db pool!"))
           .expect("Could not update the server!");

       let _ = msg.channel_id.send_message(&ctx.http, |m| {
           m.embed(|e| {
               e.title("Commands management");
               e.description(format!("Command **{}** has been {}d", args[0], args[1]));
               e.color(EMBED_REGULAR_COLOR);
               e
           });
           m
       });
       Ok(())
   }

    fn get_cmd_info(&self, ctx: &Context, msg: &Message, cmd_name: String, info: &ServerInfo) -> Result<(), String> {
        find_command(&cmd_name, info)?;
        if is_command_protected(&cmd_name)? {
            return Err("Command is protected. Enabled in every channel by default!".to_string())
        }
        let cmd = match get_db_command_by_name(info, cmd_name) {
            Some(c) => c,
            None => return Err("Could not get this command from the database".to_string())
        };

        let mut channels_message = String::new();
        cmd.enabled_channels.iter().for_each(|c| channels_message.push_str(&format!("- <#{}>\n", c)));

        let _ = msg.channel_id.send_message(&ctx.http, |m| {
           m.embed(|e| {
               e.title(format!("{} details", cmd.command_name));
               e.description(format!("Command enabled in those channels:\n{}", channels_message));
               e.color(EMBED_REGULAR_COLOR);
               e
           });
            m
        });
        Ok(())
    }
}

impl Command for CmdCommand {
    fn name(&self) -> String {
        String::from("command")
    }

    fn desc(&self) -> String {
        String::from("Managing tool for commands.")
    }

    fn use_in_dm(&self) -> bool {
        false
    }

    fn args(&self) -> Option<Vec<CommandArg>> {
        Some(vec![
            CommandArg {
                name: String::from("<command name>"),
                desc: Some(String::from("allows you to enable/disable command for provided channel.")),
                option: Some(ArgOption::Any),
                next: Some(Box::new(CommandArg {
                    name: String::from("<enable/disable>"),
                    desc: None,
                    option: Some(ArgOption::Text),
                    next: Some(Box::new(CommandArg {
                        name: String::from("<channel>"),
                        desc: None,
                        option: Some(ArgOption::Channel),
                        next: None,
                    })),
                })),
            },
            CommandArg {
                name: String::from("<command name>"),
                desc: Some(String::from("allows you to enable/disable command for this channel.")),
                option: Some(ArgOption::Any),
                next: Some(Box::new(CommandArg {
                    name: String::from("<enable/disable>"),
                    desc: None,
                    option: Some(ArgOption::Text),
                    next: None,
                })),
            },
            CommandArg {
                name: String::from("<command name>"),
                desc: Some(String::from("shows information about provided command.")),
                option: Some(ArgOption::Any),
                next: None,
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
        Some(vec!["command".to_string()])
    }

    fn config(&self) -> Option<Vec<CommandConfig>> {
        None
    }

    fn exe(&self, ctx: &Context, msg: &Message, info: &ServerInfo) -> Result<(), String> {
        let args = get_args(msg.clone(), false);
        match parse_args(&self.args().unwrap(), &args) {
            Ok(routes) => match routes {
                Some(path) => {
                    match path.len() {
                        2 => self.change_command(ctx, msg, args, info, false),
                        3 => self.change_command(ctx, msg, args, info, true),
                        _ => self.get_cmd_info(ctx, msg, args[0].to_owned(), info)
                    }
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

