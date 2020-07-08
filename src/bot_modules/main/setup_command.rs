use crate::command::{get_args, parse_args, ArgOption, Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR};
use crate::database::models::*;
use serenity::model::channel::{Message, ChannelType, PermissionOverwrite, PermissionOverwriteType};
use serenity::prelude::Context;
use crate::utils::db::{ServerInfo, get_db_roles, create_special_entity};
use crate::bot_modules::BotModule;
use serenity::model::Permissions;
use crate::utils::object_finding::get_role_from_id;
use crate::bot_modules::moderation::ModerationModule;

pub struct SetupCommand;

impl SetupCommand {
    fn create_tickets(&self, ctx: &Context, msg: &Message, info: &ServerInfo, args: Vec<String>) -> Result<(), String> {
        let role_id = if args.len() > 1 {
            match get_role_from_id(ctx, msg, get_args(msg.clone(), true), 2)? {
                Some(r) => r.id,
                None => return Ok(())
            }
        } else {
            // create role
            let result = msg.guild_id.unwrap().create_role(ctx.http.clone(), |r| {
                r.name("Support");
                r.mentionable(true);
                r.colour(2682408);
                r.hoist(true);
                r
            });

            match result {
                Ok(r) => r.id,
                Err(_) => return Err("Could not create a new role!".to_string())
            }
        };

        let result = msg.guild_id.unwrap().create_channel(ctx.http.clone(), |ch| {
            ch.name("Tickets");
            ch.topic("Tickets category. Made with UtterBot!");
            ch.kind(ChannelType::Category);
            let mut perms = Permissions::SEND_MESSAGES;
            perms.insert(Permissions::READ_MESSAGES);
            perms.insert(Permissions::ADD_REACTIONS);
            ch.permissions(vec![
                PermissionOverwrite {
                    allow: perms,
                    deny: Permissions::empty(),
                    kind: PermissionOverwriteType::Role(role_id),
                },
                PermissionOverwrite {
                    allow: Permissions::empty(),
                    deny: perms,
                    kind: PermissionOverwriteType::Role(msg.guild_id.unwrap().0.into())
                }
            ]);
            ch
        });

        match result {
            Ok(c) => create_special_entity(info, c.id.to_string(), SpecialEntityType::TicketsCategory),
            Err(_) => return Err("Could not create tickets category!".to_string())
        }

        let _ = msg.channel_id.send_message(ctx.http.clone(), |m| {
            m.embed(|e| {
                e.title("Setup - Done!");
                e.description("Tickets has been setup!");
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });
        Ok(())
    }

    fn create_mute_role(&self, ctx: &Context, msg: &Message, info: &ServerInfo, args: Vec<String>) -> Result<(), String> {
        let name = if args.len() > 1 {
            args[1].to_owned()
        } else {
            "muted".to_string()
        };

        match msg.guild(&ctx.cache) {
            Some(g) => {
                let result = g.read().create_role(ctx.http.clone(), |r| {
                    r.name(name);
                    r.mentionable(false);
                    r.permissions(Permissions::READ_MESSAGES);
                    r
                });

                match result {
                    Ok(role) => create_special_entity(info, role.id.to_string(), SpecialEntityType::MuteRole),
                    Err(_) => return Err("Could not create muted role. Do I have needed permissions?".to_string())
                }
            },
            None => return Err("Could not retrieve the guild from cache".to_string())
        }

        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Setup - Done!");
                e.description("Muted role has been created!");
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });

        Ok(())
    }

    fn create_mod_logs(&self, ctx: &Context, msg: &Message, info: &ServerInfo, args: Vec<String>) -> Result<(), String> {
        let name = if args.len() > 1 {
            args[1].to_owned()
        } else {
          "mod-logs".to_string()
        };

        let mod_module = ModerationModule{};

        let mut roles_perms = match get_db_roles(info.server.as_ref().unwrap()) {
            Some(r) => r.iter().filter(|v| {
                for p in v.perms.iter() {
                    for c in mod_module.commands() {
                        match c.perms() {
                            Some(perms) => if perms.contains(p) {
                                return true
                            }
                            None => {}
                        }
                    }
                }
                false
            }).map(|v| PermissionOverwrite {
                allow: Permissions::READ_MESSAGES,
                deny: Permissions::SEND_MESSAGES,
                kind: PermissionOverwriteType::Role(v.role_id.parse::<u64>().unwrap().into())
            }).collect(),
            None => Vec::new()
        };

        let mut read_send_perm = Permissions::READ_MESSAGES;
        read_send_perm.insert(Permissions::SEND_MESSAGES);

        roles_perms.push(PermissionOverwrite {
            allow: Permissions::empty(),
            deny: read_send_perm,
            kind: PermissionOverwriteType::Role(msg.guild_id.unwrap().0.into())
        });

        roles_perms.push(PermissionOverwrite {
            allow: read_send_perm,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(ctx.cache.read().user.id)
        });

        match msg.guild(&ctx.cache) {
            Some(g) => {
                let result = g.read().create_channel(ctx.http.clone(), |c| {
                    c.name(name);
                    c.kind(ChannelType::Text);
                    c.topic("Mod Logs - UtterBot");
                    c.permissions(roles_perms);
                    c
                });

                match result {
                    Ok(c) => {
                        create_special_entity(info, c.id.to_string(), SpecialEntityType::ModLogsChannel);
                        let _ = c.send_message(ctx.clone().http, |m| {
                            m.embed(|e| {
                                e.title("Mod Logs");
                                e.description("Every moderation related stuff will be logged here!");
                                e.color(EMBED_REGULAR_COLOR);
                                e
                            });
                            m
                        });
                    },
                    Err(_) => return Err("Could not create mod logs channel. Do I have needed permissions?".to_string())
                }
            },
            None => return Err("Could not retrieve the guild from cache".to_string())
        }

        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Setup - Done!");
                e.description("Mod logs channel has been created!");
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });

        Ok(())
    }
}

impl Command for SetupCommand {
    fn name(&self) -> String {
        String::from("setup")
    }

    fn desc(&self) -> String {
        String::from("Setup tool.")
    }

    fn use_in_dm(&self) -> bool {
        false
    }

    fn args(&self) -> Option<Vec<CommandArg>> {
        Some(vec![
            CommandArg {
                name: String::from("muted-role"),
                desc: Some(String::from("creates role used for mute command.")),
                option: Some(ArgOption::Any),
                next: Some(Box::new(CommandArg {
                    name: String::from("[name]"),
                    desc: None,
                    option: Some(ArgOption::Any),
                    next: None,
                })),
            },
            CommandArg {
                name: String::from("modlogs-channel"),
                desc: Some(String::from("creates channel used for moderation logging.")),
                option: Some(ArgOption::Any),
                next: Some(Box::new(CommandArg {
                    name: String::from("[name]"),
                    desc: None,
                    option: Some(ArgOption::Any),
                    next: None,
                })),
            },
            CommandArg {
                name: String::from("tickets"),
                desc: Some(String::from("creates tickets category and support role (unless provided)")),
                option: Some(ArgOption::Any),
                next: Some(Box::new(CommandArg {
                    name: String::from("[support-role]"),
                    desc: None,
                    option: Some(ArgOption::Role),
                    next: None
                })),
            },
            CommandArg {
                name: String::from(""),
                desc: Some(String::from("shows usage information.")),
                option: None,
                next: None,
            },
        ])
    }

    fn perms(&self) -> Option<Vec<String>> {
        Some(vec!["setup".to_string()])
    }

    fn config(&self) -> Option<Vec<CommandConfig>> {
        None
    }

    fn exe(&self, ctx: &Context, msg: &Message, info: &ServerInfo) -> Result<(), String> {
        let args = get_args(msg.clone(), false);
        match parse_args(&self.args().unwrap(), &args) {
            Ok(routes) => match routes {
                Some(path) => {
                    match path[0].name.as_str() {
                        "modlogs-channel" => self.create_mod_logs(ctx, msg, info, args)?,
                        "muted-role" => self.create_mute_role(ctx, msg, info, args)?,
                        "tickets" => self.create_tickets(ctx, msg, info, args)?,
                        _ => return Err("Not implemented".to_string())
                    }

                    Ok(())
                }
                None => {
                    let help_cmd = super::help_command::HelpCommand{};
                    help_cmd.show_cmd_details(ctx, msg, info, self.name())
                },
            },
            Err(why) => return Err(why),
        }
    }
}

