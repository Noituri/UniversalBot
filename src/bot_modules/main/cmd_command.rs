use super::super::*;
use crate::command::{get_args, parse_args, ArgOption, Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR, find_command};
use crate::config::DEV_MODULE;
use crate::database::get_db_con;
use crate::database::models::*;
use crate::database::schema::commands::enabled_channels;
use crate::database::schema::*;
use crate::utils::check_if_dev;
use diesel::prelude::*;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::utils::db::get_db_command;

pub struct CmdCommand;

impl CmdCommand {
   fn change_command(&self, ctx: &Context, msg: &Message, args: Vec<String>, srv: Server, is_channel: bool) -> Result<(), String> {
       find_command(&args[0], &srv)?;
       let channel = if is_channel {
           args[2].to_owned()
       } else {
           msg.channel_id.0.to_string()
       };

       let mut cmd = if let Some(c) = get_db_command(msg.guild_id, args[0].to_string()) {
           c
       } else {
           return Err("Could not find command in the database!".to_string())
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
}

impl Command for CmdCommand {
    fn name(&self) -> String {
        String::from("command")
    }

    fn desc(&self) -> String {
        String::from("Managing tool for commands.")
    }

    fn enabled(&self) -> bool {
        true
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

    fn exe(&self, ctx: &Context, msg: &Message, server: Option<Server>) -> Result<(), String> {
        let args = get_args(msg.clone());
        match parse_args(&self.args().unwrap(), &args) {
            Ok(routes) => match routes {
                Some(path) => {
                    let s = server.unwrap();
                    match path.len() {
                        2 => self.change_command(ctx, msg, args, s, false),
                        3 => self.change_command(ctx, msg, args, s, true),
                        _ => Ok(())
                    }
                }
                None => Ok(()),
            },
            Err(why) => return Err(why),
        }
    }
}

