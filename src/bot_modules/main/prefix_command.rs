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
use crate::utils::db::ServerInfo;

pub struct PrefixCommand;

impl PrefixCommand {
    fn show_prefix(&self, ctx: &Context, msg: &Message, info: &ServerInfo) -> Result<(), String> {
        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Prefix");
                e.description(format!("Current prefix: `{}`", info.server.as_ref().unwrap().prefix));
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });
        Ok(())
    }

    fn set_prefix(&self, ctx: &Context, msg: &Message, info: &ServerInfo, new_prefix: &str) -> Result<(), String> {
        if new_prefix.trim() == "" {
            return Err(String::from("Prefix can't be empty!"));
        }

        let db = get_db_con().get().expect("Could not get db pool!");

        let server = info.server.as_ref().unwrap();
        let result = diesel::update(servers::dsl::servers.find(server.id))
            .set(prefix.eq(new_prefix))
            .get_result::<Server>(&db);

        if result.is_err() {
            return Err(String::from("Could not update the server config"));
        }

        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("New prefix had been set!");
                e.description(format!("Current prefix: `{}`", new_prefix));
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });

        Ok(())
    }
}

impl Command for PrefixCommand {
    fn name(&self) -> String {
        String::from("prefix")
    }

    fn desc(&self) -> String {
        String::from("Prefix manager.")
    }

    fn use_in_dm(&self) -> bool {
        false
    }

    fn args(&self) -> Option<Vec<CommandArg>> {
        Some(vec![
            CommandArg {
                name: String::from("set"),
                desc: Some(String::from("sets custom bot prefix for server.")),
                option: Some(ArgOption::Numeric),
                next: Some(Box::new(CommandArg {
                    name: String::from("<prefix>"),
                    desc: None,
                    option: Some(ArgOption::Text),
                    next: None,
                })),
            },
            CommandArg {
                name: String::from(""),
                desc: Some(String::from("shows bot prefix.")),
                option: None,
                next: None,
            },
        ])
    }

    fn perms(&self) -> Option<Vec<String>> {
        Some(vec!["prefix".to_string()])
    }

    fn config(&self) -> Option<Vec<CommandConfig>> {
        None
    }

    fn exe(&self, ctx: &Context, msg: &Message, info: &ServerInfo) -> Result<(), String> {
        let args = get_args(msg.clone());
        match parse_args(&self.args().unwrap(), &args) {
            Ok(routes) => {
                match routes {
                    Some(path) => {
                        if path.len() == 0 {
                            self.show_prefix(ctx, msg, info)?;
                        } else {
                            self.set_prefix(ctx, msg, info, &args[1])?;
                        }
                    }
                    None => return self.show_prefix(ctx, msg, info),
                }
            }
            Err(why) => return Err(why),
        }
        Ok(())
    }
}
