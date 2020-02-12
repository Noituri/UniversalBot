use crate::command::{Command, CommandConfig, EMBED_REGULAR_COLOR, CommandArg, get_args, parse_args, ArgOption};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::database::models::Server;
use crate::database::schema::servers;
use crate::database::schema::servers::columns::prefix;
use diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};
use crate::database::get_db_con;

pub struct PermsCommand;

impl PermsCommand {

}

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
                    Some(path) => {

                    }
                    None => {}
                }
            }
            Err(why) => return Err(why)
        }
        Ok(())
    }
}