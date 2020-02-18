use crate::command::{
    get_args, parse_args, ArgOption, Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR,
};
use crate::database::get_db_con;
use crate::database::models::Role;
use crate::database::schema::roles;
use diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::database::schema::roles::columns::perms;
use crate::utils::db::{ServerInfo, get_db_role_by_id};
use crate::utils::object_finding::get_role_from_id;
use crate::utils::perms::{get_module_perms, perms_exists};

pub struct BanCommand;

impl Command for BanCommand {
    fn name(&self) -> String {
        String::from("ban")
    }

    fn desc(&self) -> String {
        String::from("Banish users from your server.")
    }

    fn use_in_dm(&self) -> bool {
        false
    }

    fn args(&self) -> Option<Vec<CommandArg>> {
        Some(vec![
            CommandArg {
                name: "<user>".to_string(),
                desc: Some("bans user. If `[time]` is provided then user will be temporally banned.\
                By default time is in days but you can change that by adding `s` for seconds, \
                `m` for minutes, `h` for hours and `d` for days behind `time`. For example `10m`.".to_string()),
                option: Some(ArgOption::User),
                next: Some(Box::new(CommandArg {
                    name: "[time]".to_string(),
                    desc: None,
                    option: Some(ArgOption::Time),
                    next: Some(Box::new(CommandArg {
                        name: "[reason...]".to_string(),
                        desc: None,
                        option: Some(ArgOption::Any),
                        next: None,
                    }))
                }))
            },
            CommandArg {
                name: "".to_string(),
                desc: Some("shows usage information".to_string()),
                option: None,
                next: None
            }
        ])
    }

    fn perms(&self) -> Option<Vec<String>> {
        Some(vec!["ban".to_string()])
    }

    fn config(&self) -> Option<Vec<CommandConfig>> {
        None
    }

    fn exe(&self, ctx: &Context, msg: &Message, info: &ServerInfo) -> Result<(), String> {
        let args = get_args(msg.clone(), false);
        match parse_args(&self.args().unwrap(), &args) {
            Ok(routes) => {
                match routes {
                    Some(path) => {

                    }
                    None => {

                    }
                }
            }
            Err(why) => return Err(why),
        }
        Ok(())
    }
}
