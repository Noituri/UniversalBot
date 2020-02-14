use crate::command::{
    get_args, parse_args, ArgOption, Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR,
};
use crate::database::get_db_con;
use crate::database::models::{Server, Role};
use crate::database::schema::{servers, roles};
use crate::database::schema::servers::columns::prefix;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::utils::{get_role_from_id, get_role, perms_exists};
use crate::database::schema::roles::columns::perms;

pub struct PermsCommand;

impl PermsCommand {
    fn add_perm (&self, ctx: &Context, msg: &Message, args: Vec<String>) -> Result<(), String> {
        let mut perms_to_add = args[2..].to_vec();
        if !perms_exists(&perms_to_add) {
            return Err("One of the provided permissions does not exist!".to_string())
        }

        let role = if let Some(r) = get_role_from_id(ctx, msg, args[1].to_owned())? {
            r
        } else {
            return Ok(())
        };

        let mut db_role = if let Some(db_r) = get_role(msg.guild_id, role.id.to_string()) {
            db_r
        } else {
            return Err("Could not find role in the database!".to_string())
        };

        for (i, p) in perms_to_add.iter().enumerate() {
            if !db_role.perms.contains(p) {
                db_role.perms.push(p.to_owned());
            }
        }

        diesel::update(roles::dsl::roles.find(db_role.id))
            .set(perms.eq(db_role.perms))
            .get_result::<Role>(&get_db_con().get().expect("Could not get db pool!"))
            .expect("Could not update the server!");

        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Permissions System");
                e.color(EMBED_REGULAR_COLOR);
                e.description(format!("Successfully updated permissions for **{}**", role.name));
                e
            });
            m
        });

        Ok(())
    }
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
            CommandArg {
                name: "get".to_string(),
                desc: Some("gets role's permissions".to_string()),
                option: None,
                next: Some(Box::new(CommandArg {
                    name: "<role>".to_string(),
                    desc: None,
                    option: Some(ArgOption::Role),
                    next: None
                })),
            },
            CommandArg{ // TODO: show user perms
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
                    Some(path) => {
                        match path[0].name.as_str() {
                            "add" => self.add_perm(ctx, msg, args)?,
                            "remove" => {},
                            _ => {}
                        }
                    }
                    None => {}
                }
            }
            Err(why) => return Err(why),
        }
        Ok(())
    }
}
