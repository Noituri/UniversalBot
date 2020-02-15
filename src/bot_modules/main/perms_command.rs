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
use crate::utils::{get_role_from_id, get_db_role, perms_exists, get_module_perms};
use crate::database::schema::roles::columns::perms;

pub struct PermsCommand;

enum PermModifyOption {
    Add,
    Remove,
    Set,
}

impl PermsCommand {
    fn get_role_perms(&self, ctx: &Context, msg: &Message, args: Vec<String>) -> Result<(), String> {
        let role = if let Some(r) = get_role_from_id(ctx, msg, args[1].to_owned())? {
            r
        } else {
            return Ok(())
        };

        let mut db_role = if let Some(db_r) = get_db_role(msg.guild_id, role.id.to_string()) {
            db_r
        } else {
            return Err("Could not find role in the database!".to_string())
        };

        let mut perms_message = String::new();
        db_role.perms.iter().for_each(|p| perms_message.push_str(&format!("- {}\n", p.to_uppercase())));

        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(role.name + " permissions");
                e.color(EMBED_REGULAR_COLOR);
                e.description(perms_message);
                e
            });
            m
        });
        Ok(())
    }

    fn modify_perm (&self, ctx: &Context, msg: &Message, args: Vec<String>, modify_option: PermModifyOption) -> Result<(), String> {
        let mut perms_to_modify = args[2..].to_vec();
        if !perms_exists(&perms_to_modify) {
            let mut modules_perms = Vec::new();
            for p in perms_to_modify.iter() {
                if let Some(m_perms) = get_module_perms(p) {
                    modules_perms.extend(m_perms.iter().cloned());
                } else {
                    return Err("One of the provided permissions or modules does not exist!".to_string())
                }
            }

            perms_to_modify = modules_perms;
        }

        let role = if let Some(r) = get_role_from_id(ctx, msg, args[1].to_owned())? {
            r
        } else {
            return Ok(())
        };

        let mut db_role = if let Some(db_r) = get_db_role(msg.guild_id, role.id.to_string()) {
            db_r
        } else {
            return Err("Could not find role in the database!".to_string())
        };

        match modify_option {
            PermModifyOption::Add => {
                for (i, p) in perms_to_modify.iter().enumerate() {
                    if !db_role.perms.contains(p) {
                        db_role.perms.push(p.to_owned());
                    }
                }
            },
            PermModifyOption::Remove => db_role.perms.retain(|v| !perms_to_modify.contains(v)),
            PermModifyOption::Set => db_role.perms = perms_to_modify
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
        String::from("permission management. You can provide permissions you want to add/remove/set to role or \
        just type module name and it will add/remove/set every permission from that module")
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
                        name: "<permissions or modules...>".to_string(),
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
                        name: "<permissions or modules...>".to_string(),
                        desc: None,
                        option: Some(ArgOption::Text),
                        next: None
                    })),
                })),
            },
            CommandArg {
                name: "set".to_string(),
                desc: Some("sets permissions for role".to_string()),
                option: None,
                next: Some(Box::new(CommandArg {
                    name: "<role>".to_string(),
                    desc: None,
                    option: Some(ArgOption::Role),
                    next: Some(Box::new(CommandArg{
                        name: "<permissions or modules...>".to_string(),
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
            CommandArg{
                name: "".to_string(),
                desc: Some("Shows usage information".to_string()),
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
                let srv = server.clone().unwrap();
                match routes {
                    Some(path) => {
                        match path[0].name.as_str() {
                            "add" => self.modify_perm(ctx, msg, args, PermModifyOption::Add)?,
                            "remove" => self.modify_perm(ctx, msg, args, PermModifyOption::Remove)?,
                            "get" => self.get_role_perms(ctx, msg, args)?,
                            _ => self.modify_perm(ctx, msg, args, PermModifyOption::Set)?
                        }
                    }
                    None => {
                        let help_cmd = super::help_command::HelpCommand{};
                        help_cmd.show_cmd_details(ctx, msg, server, self.name());
                    }
                }
            }
            Err(why) => return Err(why),
        }
        Ok(())
    }
}
