use crate::command::{Command, CommandConfig, EMBED_REGULAR_COLOR, CommandArg, get_args, parse_args, ArgOption};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::Error;
use crate::database::models::Server;
use crate::config::VERSION;

pub struct HelpCommand;

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
                desc: Some(String::from("shows 10 first commands from every module")),
                option: Some(ArgOption::Numeric),
                next: None
            },
            CommandArg{
                name: String::from("all"),
                desc: Some(String::from("shows 10 first commands from every module for given page")),
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
                desc: Some(String::from("shows 10 first commands of given page")),
                option: Some(ArgOption::Any),
                next: None
            },
            CommandArg{
                name: String::from(""),
                desc: Some(String::from("shows 10 first commands")),
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
        Ok(())
    }
}