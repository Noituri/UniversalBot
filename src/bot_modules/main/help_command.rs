use crate::command::{Command, CommandConfig, EMBED_REGULAR_COLOR, CommandArg, get_args, parse_args, ArgOption};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::Error;
use crate::database::models::Server;
use crate::config::{VERSION, DEFAULT_PREFIX};
use crate::bot_modules::get_modules;
use crate::utils::has_perms;

pub struct HelpCommand;

impl HelpCommand {
    fn show_help(&self, ctx: &Context, msg: &Message, server: Option<Server>, all: bool) -> Result<(), String> {
        let prefix =
            if let Some(s) = server {
                s.prefix
            } else {
                DEFAULT_PREFIX
            };

        let usage_message = format!(
            "**Usage:**\n\
            {0}help - shows enabled commands from enabled modules\n\
            {0}help <page> - shows 10 first commands of given page\n\
            {0}help all - shows commands from enabled and disabled modules\n\
            {0}help all <page> - shows commands from enabled and disabled modules for given page\n\
            {0}help <command> - shows details about command\n\n\
            ",
            prefix
        );

        let commands_message = String::new();
        let mut commands = Vec::new();
        for m in get_modules().iter() {
            for c in m.commands().iter() {
                if c.use_in_dm() && server.is_none() {
                    commands.push(&c);
                    continue;
                }

                if let Some(s) = server {
                    if has_perms(&ctx, &msg,s.clone(), &c.perms()) {
                        commands.push(&c);
                        continue;
                    }
                }
            }
        }

        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Help");
                e.description();
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });
        Ok(())
    }
}

impl Command for HelpCommand {
    fn name(&self) -> String {
        String::from("help")
    }

    fn desc(&self) -> String {
        String::from("shows this help message.")
    }

    fn enabled(&self) -> bool {
        // TODO: make it disable-able
        true
    }

    fn use_in_dm(&self) -> bool {
        true
    }

    fn args(&self) -> Option<Vec<CommandArg>> {
        Some(vec![
            CommandArg{
                name: String::from("all"),
                desc: Some(String::from("shows commands from enabled and disabled modules")),
                option: Some(ArgOption::Numeric),
                next: None
            },
            CommandArg{
                name: String::from("all"),
                desc: Some(String::from("shows commands from enabled and disabled modules for given page")),
                option: Some(ArgOption::Numeric),
                next: Some(
                    Box::new(
                        CommandArg{
                            name: String::from("<page>"),
                            desc: None,
                            option: Some(ArgOption::Numeric),
                            next: None
                        },
                    )
                )
            },
            CommandArg{
                name: String::from("<page>"),
                desc: Some(String::from("shows 10 first commands of given page")),
                option: Some(ArgOption::Numeric),
                next: None
            },
            CommandArg{
                name: String::from("<command>"),
                desc: Some(String::from("shows details about command")),
                option: Some(ArgOption::Any),
                next: None
            },
            CommandArg{
                name: String::from(""),
                desc: Some(String::from("shows enabled commands from enabled modules")),
                option: None,
                next: None
            }
        ])
    }

    fn perms(&self) -> Option<Vec<String>> {
        None
    }

    fn config(&self) -> Option<Vec<CommandConfig>> {
        None
    }

    fn exe(&self, ctx: &Context, msg: &Message, server: Option<Server>) -> Result<(), String> {
        let args = get_args(msg.clone());
        match parse_args(&self.args().unwrap(), &args) {
            Ok(routes) => {
                match routes {
                    Some(path) => {
                        let s = server.unwrap();
                        match path.len() {
                            1 => return self.show_module_details(&ctx, &msg, &args),
                            2 => {
                                if args[1] == "commands" {
                                    return self.module_commands(&ctx, &msg, &args, &s.prefix);
                                }
                                return self.enable_module(&ctx, &msg, &args, s);
                            },
                            _ => return Err(String::from("Too many args!"))
                        }
                    }
                    None => return self.show_modules(&ctx, &msg)
                }
            }
            Err(why) => return Err(why)
        }
        Ok(())
    }
}