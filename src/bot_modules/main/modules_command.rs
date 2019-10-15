use crate::command::{Command, CommandPerms, CommandConfig, EMBED_REGULAR_COLOR, CommandArg, parse_args, get_args};
use super::super::bot_modules;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::bot_modules::bot_modules::BotModule;
use serenity::Error;

pub struct ModulesCommand;

impl ModulesCommand {
    fn show_modules(&self, ctx: &Context, msg: &Message) -> Result<Message, Error> {
        let mut modules_str = String::new();
        for m in bot_modules::get_modules().iter() {
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
        })
    }

    fn module_commands(&self, ctx: &Context, msg: &Message) -> Result<Message, Error> {
        let mut commands_str = String::new();
        for m in super::MainModule.commands().iter() {
            commands_str += &format!("**{}** - {}\n", m.name(), m.desc());
        }

        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Commands");
                e.description(commands_str);
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        })
    }

    fn enable_module(&self, ctx: &Context, msg: &Message) -> Result<Message, Error> {
        Ok(msg.clone())
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
                        name: String::from("set"),
                        desc: None,
                        optional: true,
                        next: Some(
                            Box::new(CommandArg{
                                name: String::from("<enable/disable>"),
                                desc: None,
                                optional: false,
                                next: None
                            })
                        )
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

    fn perms(&self) -> Option<Vec<CommandPerms>> {
        Some(vec![CommandPerms::Modules])
    }

    fn config(&self) -> Option<Vec<CommandConfig>> {
        None
    }

    fn exe(&self, ctx: &Context, msg: &Message) -> Result<Message, Error> {
        let a = parse_args(self.args().unwrap(), get_args(msg.clone()));

        match a {
            Ok(route) => {
                match route {
                    Some(r) => {
                        r.iter().for_each(|x| println!("ROUTE: {}", x.name));
                    }
                    None => println!("NO ROUTE")
                }
            }
            Err(_) => {
                println!("NO MATCHING ROUTE");
            }
        }
        Ok(msg.clone())
    }
}