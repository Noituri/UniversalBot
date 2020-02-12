use crate::command::{Command, CommandConfig, EMBED_REGULAR_COLOR, CommandArg, get_args, parse_args, ArgOption};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::database::models::Server;
use crate::database::schema::servers;
use crate::database::schema::servers::columns::prefix;
use diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};
use crate::database::get_db_con;

pub struct PrefixCommand;

impl PrefixCommand {
    fn show_prefix(&self, ctx: &Context, msg: &Message, server: &Server) -> Result<(), String> {
        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Prefix");
                e.description(format!("Current prefix: `{}`", server.prefix));
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });
        Ok(())
    }

    fn set_prefix(&self, ctx: &Context, msg: &Message, server: &Server, new_prefix: &str) -> Result<(), String> {
        if new_prefix.trim() == "" {
            return Err(String::from("Prefix can't be empty!"))
        }

        let db = get_db_con().get().expect("Could not get db pool!");

        let result = diesel::update(servers::dsl::servers.find(server.id))
            .set(prefix.eq(new_prefix))
            .get_result::<Server>(&db);

        if result.is_err() {
            return Err(String::from("Could not update the server config"))
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
        String::from("prefix management.")
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
                name: String::from("set"),
                desc: Some(String::from("sets custom bot prefix for server.")),
                option: Some(ArgOption::Numeric),
                next: Some(
                    Box::new(
                        CommandArg{
                            name: String::from("<prefix>"),
                            desc: None,
                            option: Some(ArgOption::Text),
                            next: None
                        },
                    )
                )
            },
            CommandArg{
                name: String::from(""),
                desc: Some(String::from("shows bot prefix.")),
                option: None,
                next: None
            }
        ])
    }

    fn perms(&self) -> Option<Vec<String>> {
        Some(vec!["prefix".to_string()])
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
                        if path.len() == 0 {
                            self.show_prefix(ctx, msg, &srv)?;
                        } else {
                            self.set_prefix(ctx, msg, &srv, &args[1])?;
                        }
                    }
                    None => return self.show_prefix(ctx, msg, &srv)
                }
            }
            Err(why) => return Err(why)
        }
        Ok(())
    }
}