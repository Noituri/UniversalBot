use crate::command::{Command, CommandConfig, EMBED_REGULAR_COLOR, CommandArg, parse_args, get_args};
use crate::database::schema::*;
use super::super::*;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::Error;
use diesel::{PgExpressionMethods, TextExpressionMethods};
use crate::database::schema::servers::columns::{guildid, enabledmodules};
use crate::database::models::*;
use diesel::prelude::*;
use std::ops::Deref;
use crate::database::get_db_con;
use crate::utils::{create_server};
use crate::config::{DEFAULT_PREFIX as PREFIX, DEFAULT_PREFIX};

pub struct ModulesCommand;

impl ModulesCommand {
    fn show_modules(&self, ctx: &Context, msg: &Message) -> Result<(), String> {
        let mut modules_str = String::new();
        for m in get_modules().iter() {
            modules_str += &format!("**{}** - {}\n", m.name(), m.desc());
        }

        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Modules");
                e.description(modules_str);
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });
        Ok(())
    }

    fn show_module_details(&self, ctx: &Context, msg: &Message, args: &Vec<String>) -> Result<(), String> {
        let module = find_module(&args[0])?;
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("Module - {}", module.name()));
                e.description(format!(r#"
                    **Description:**
                    {}

                    **Enabled:** {}
                "#, module.desc(), module.enabled()));
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });
        Ok(())
    }

    fn module_commands(&self, ctx: &Context, msg: &Message, args: &Vec<String>, prefix: &str) -> Result<(), String> {
        let module = find_module(&args[0])?;
        let mut commands_str = String::new();
        for m in module.commands().iter() {
            commands_str += &format!("**{}{}** - {}\n", prefix, m.name(), m.desc());
        }

        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Commands");
                e.description(commands_str);
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });
        Ok(())
    }

    fn enable_module(&self, ctx: &Context, msg: &Message, args: &Vec<String>, mut server: Server) -> Result<(), String> {
        let _ = find_module(&args[0])?;
        if PROTECTED_MODULES.contains(&args[0].as_str()) {
            return Err(String::from("This module is protected. It means that it can't be enabled or disabled."));
        }

        let db = get_db_con().get().expect("Could not get db pool!");
        if args[1] == "enable" && !server.enabledmodules.contains(&args[0]) {
            server.enabledmodules.push(args[0].to_owned())
        } else if args[1] == "disable" {
            for (i, m) in server.enabledmodules.iter().enumerate() {
                if m == &args[0] {
                    server.enabledmodules.remove(i);
                    break;
                }
            }
        }

        diesel::update(servers::dsl::servers.find(server.id))
            .set(enabledmodules.eq(&server.enabledmodules))
            .get_result::<Server>(&db)
            .expect("Could not update the server!");

        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Module management");
                e.description(format!("Module {} has been {}d", &args[0], args[1]));
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });
        Ok(())
    }
}

impl Command for ModulesCommand {
    fn name(&self) -> String {
        String::from("modules")
    }

    fn desc(&self) -> String {
        String::from("shows every module in this bot.")
    }

    fn enabled(&self) -> bool {
        true
    }

    fn use_in_dm(&self) -> bool {
        false
    }

    fn args(&self) -> Option<Vec<CommandArg>> {
        Some(vec![
            CommandArg{
                name: String::from("<module name>"),
                desc: Some(String::from("shows every command available in provided module.")),
                optional: false,
                next: Some(
                    Box::new(CommandArg{
                        name: String::from("commands"),
                        desc: None,
                        optional: true,
                        next: None
                    })
                )
            },
            CommandArg{
                name: String::from("<module name>"),
                desc: Some(String::from("allows you to enable/disable module.")),
                optional: false,
                next: Some(
                    Box::new(CommandArg{
                        name: String::from("<enable/disable>"),
                        desc: None,
                        optional: false,
                        next: None
                    })
                )
            },
            CommandArg{
                name: String::from("<module name>"),
                desc: Some(String::from("shows information about provided module")),
                optional: false,
                next: None
            },
            CommandArg{
                name: String::from(""),
                desc: Some(String::from("shows information about every module")),
                optional: false,
                next: None
            },
        ])
    }

    fn perms(&self) -> Option<Vec<String>> {
        Some(vec!["modules".to_string()])
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