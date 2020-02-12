use crate::command::{Command, CommandConfig, EMBED_REGULAR_COLOR, CommandArg, get_args, parse_args, ArgOption};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::database::models::Server;
use crate::config::{DEFAULT_PREFIX};
use crate::bot_modules::get_modules;
use crate::utils::has_perms;

pub struct HelpCommand;

impl HelpCommand {
    fn show_help(&self, ctx: &Context, msg: &Message, server: Option<Server>, all: bool, page: usize) -> Result<(), String> {
        if page == 0 {
            return Err(String::from("Page does not exist!"));
        }

        let prefix =
            if let Some(s) = server.clone() {
                s.prefix
            } else {
                DEFAULT_PREFIX.to_string()
            };

        let usage_message = if page == 1 {
            format!(
                "**Usage:**\n\
                **{0}help** - shows enabled commands from enabled modules\n\
                **{0}help** <page> - shows 10 first commands of given page\n\
                **{0}help** all - shows commands from enabled and disabled modules\n\
                **{0}help** all <page> - shows commands from enabled and disabled modules for given page\n\
                **{0}help** <command> - shows details about command\n\n\
                ",
                prefix
            )
        } else {
            String::new()
        };

        let mut commands_message = String::from("**Commands:**\n");
        let mut commands = Vec::new();
        for m in get_modules().iter() {

            for c in m.commands() {
                if all || (c.use_in_dm() && server.is_none()) {
                    commands.push(c);
                    continue;
                }
                if let Some(s) = &server {
                    if all || has_perms(&ctx, &msg,s.clone(), &c.perms()) {
                        commands.push(c);
                        continue;
                    }
                }
            }
        }

        let start_page = if (page-1)*10 > commands.len() {
            return Err(String::from("Page does not exist"));
        } else {
            (page-1)*10
        };

        let end_page = if (page-1)*10 + 10 > commands.len() {
            commands.len()
        } else {
            (page-1)*10 + 10
        };

        commands.sort_by(|a, b| a.name().to_lowercase().cmp(&b.name().to_lowercase()));
        for c in commands[start_page..end_page].iter() {
            commands_message.push_str(&format!("**{}{}** - {}\n", prefix, c.name(), c.desc()));
        }

        let mut pages_number = commands.len() / 10;
        if commands.len() % 10 != 0 {
            pages_number += 1;
        }

        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Help");
                e.description(format!("{}{}", usage_message, commands_message));
                e.footer(|f| {
                    f.text(format!("Page: {}/{}", page, pages_number));
                    f
                });
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });
        Ok(())
    }

    fn show_cmd_details(&self, ctx: &Context, msg: &Message, server: Option<Server>, cmd_name: String) -> Result<(), String> {
        let prefix =
            if let Some(s) = server.clone() {
                s.prefix
            } else {
                DEFAULT_PREFIX.to_string()
            };

        for m in get_modules() {
            for c in m.commands() {
                if c.name() == cmd_name {
                    let mut args_message = String::new();
                    if let Some(_args) = c.args() {
                        args_message = String::from("**Arguments:**\n");
                        for a in c.args().unwrap().iter() {
                            let mut next_arg = a.next.as_ref();
                            let mut options_message = a.name.to_owned();
                            while next_arg.is_some() {
                                let na = next_arg.unwrap();
                                options_message.push_str(&format!(" {}", na.name));
                                next_arg = na.next.as_ref();
                            }
                            args_message.push_str(&format!("**{}{} {}** - {}\n",
                                                           prefix,
                                                           c.name(),
                                                           options_message,
                                                           a.desc.as_ref().unwrap())
                            );
                        }
                    }

                    let mut perms_message = String::from("**Permissions:**\n");
                    if let Some(perms) = c.perms() {
                        for p in perms.iter() {
                            perms_message.push_str(&format!("- {}\n", p));
                        }

                        perms_message.push_str(&format!("\nUse `{}perms add <@role> <permission>` to add permissions to the role.", prefix))
                    } else {
                        perms_message.push_str("No extra permissions needed.\n");
                    }

                    let _ = msg.channel_id.send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title("Help - Command details");
                            e.description(
                                format!("**Name: ** {}\n\
                                **Description:** {}\n\
                                **Enabled:** {} \n\
                                **Can be used in DM:** {}\n\n\
                                {}\n\
                                {}\
                                ", c.name(), c.desc(), c.enabled(), c.use_in_dm(), args_message, perms_message)
                            );
                            e.color(EMBED_REGULAR_COLOR);
                            e
                        });
                        m
                    });
                    return Ok(())
                }
            }
        }
        Err(String::from("Command not found!"))
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
        true
    }

    fn use_in_dm(&self) -> bool {
        true
    }

    fn args(&self) -> Option<Vec<CommandArg>> {
        Some(vec![
            CommandArg{
                name: String::from("all"),
                desc: Some(String::from("shows commands from enabled and disabled modules.")),
                option: Some(ArgOption::Numeric),
                next: None
            },
            CommandArg{
                name: String::from("all"),
                desc: Some(String::from("shows commands from enabled and disabled modules for given page.")),
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
                desc: Some(String::from("shows 10 first commands of given page.")),
                option: Some(ArgOption::Numeric),
                next: None
            },
            CommandArg{
                name: String::from("<command>"),
                desc: Some(String::from("shows details about command.")),
                option: Some(ArgOption::Any),
                next: None
            },
            CommandArg{
                name: String::from(""),
                desc: Some(String::from("shows enabled commands from enabled modules.")),
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
                        match path.len() {
                            1 => if path[0].name == "all" {
                                return self.show_help(&ctx, &msg, server.clone(), true, 1)
                            } else {
                                match args[0].parse::<usize>() {
                                    Ok(p) => return self.show_help(&ctx, &msg, server.clone(), false, p),
                                    Err(_) => return self.show_cmd_details(&ctx, &msg, server.clone(), args[0].to_owned())
                                }
                            }
                            2 => if path[0].name == "all" {
                                match args[1].parse::<usize>() {
                                    Ok(p) => return self.show_help(&ctx, &msg, server.clone(), true, p),
                                    Err(_) => return Err(String::from("Invalid page number!"))
                                }
                            }
                            _ => return Err(String::from("Too many args!"))
                        }
                        if path.len() == 1 && path[0].name == "all" {
                            return self.show_help(&ctx, &msg, server.clone(), true, 1)
                        }
                    }
                    None => return self.show_help(&ctx, &msg, server.clone(), false, 1)
                }
            }
            Err(why) => return Err(why)
        }
        Ok(())
    }
}