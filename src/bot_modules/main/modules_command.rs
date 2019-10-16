use crate::command::{Command, CommandPerms, CommandConfig, EMBED_REGULAR_COLOR, CommandArg, parse_args, get_args};
use super::super::bot_modules;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::bot_modules::bot_modules::BotModule;
use serenity::Error;

pub struct ModulesCommand;

impl ModulesCommand {
    fn show_modules(&self, ctx: &Context, msg: &Message) -> Result<(), String> {
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
        });
        Ok(())
    }

    fn show_module_details(&self, ctx: &Context, msg: &Message, args: &Vec<String>) -> Result<(), String> {
        let module = bot_modules::find_module(&args[0])?;
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

    fn module_commands(&self, ctx: &Context, msg: &Message, args: &Vec<String>) -> Result<(), String> {
        let module = bot_modules::find_module(&args[0])?;
        let mut commands_str = String::new();
        for m in module.commands().iter() {
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
        });
        Ok(())
    }

    fn enable_module(&self, ctx: &Context, msg: &Message, args: &Vec<String>) -> Result<(), String> {
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

    fn exe(&self, ctx: &Context, msg: &Message) -> Result<(), String> {
        let args = get_args(msg.clone());

        match parse_args(self.args().unwrap(), &args) {
            Ok(routes) => {
                match routes {
                    Some(path) => {
                        match path.len() {
                            1 => return self.show_module_details(&ctx, &msg, &args),
                            2 => return self.module_commands(&ctx, &msg, &args),
                            3 => return self.enable_module(&ctx, &msg, &args),
                            _ => return Err(String::from("Too many args!"))
                        }
                        path.iter().for_each(|x| println!("ROUTE: {}", x.name));
                    }
                    None => return self.show_modules(&ctx, &msg)
                }
            }
            Err(why) => return Err(why)
        }
        Ok(())
    }
}