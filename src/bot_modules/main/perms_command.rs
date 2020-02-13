use crate::command::{
    get_args, parse_args, ArgOption, Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR,
};
use crate::database::get_db_con;
use crate::database::models::Server;
use crate::database::schema::servers;
use crate::database::schema::servers::columns::prefix;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serenity::model::channel::Message;
use serenity::prelude::Context;

pub struct PermsCommand;

impl PermsCommand {}

impl Command for PermsCommand {
    fn name(&self) -> String {
        String::from("perms")
    }

    fn desc(&self) -> String {
        String::from("permission management.")
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
                name: "add".to_string(),
                desc: Some("adds permissions to role".to_string()),
                option: None,
                next: Some(Box::new(CommandArg {
                    name: "<role>".to_string(),
                    desc: None,
                    option: Some(ArgOption::Role),
                    next: Some(Box::new(CommandArg{
                        name: "<permissions...>".to_string(),
                        desc: None,
                        option: Some(ArgOption::Text),
                        next: None
                    })),
                })),
            },
            CommandArg {
                name: "remove".to_string(),
                desc: Some("removes permissions from role".to_string()),
                option: None,
                next: Some(Box::new(CommandArg {
                    name: "<role>".to_string(),
                    desc: None,
                    option: Some(ArgOption::Role),
                    next: Some(Box::new(CommandArg{
                        name: "<permissions...>".to_string(),
                        desc: None,
                        option: Some(ArgOption::Text),
                        next: None
                    })),
                })),
            },
            CommandArg{
                name: "".to_string(),
                desc: None,
                option: None,
                next: None
            }
        ])
    }

    fn perms(&self) -> Option<Vec<String>> {
        Some(vec!["perms".to_string()])
    }

    fn config(&self) -> Option<Vec<CommandConfig>> {
        None
    }

    fn exe(&self, ctx: &Context, msg: &Message, server: Option<Server>) -> Result<(), String> {
        let args = get_args(msg.clone());
        match parse_args(&self.args().unwrap(), &args) {
            Ok(routes) => {
                let srv = server.unwrap();
                match routes {
                    Some(path) => {}
                    None => {}
                }
            }
            Err(why) => return Err(why),
        }
        Ok(())
    }
}
