use crate::command::{get_args, parse_args, ArgOption, Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR};
use crate::database::models::*;
use serenity::model::channel::{Message, ChannelType, PermissionOverwrite, PermissionOverwriteType};
use serenity::prelude::Context;
use crate::utils::db::{ServerInfo, get_db_roles, create_special_entity};
use crate::bot_modules::BotModule;
use serenity::model::Permissions;
use crate::bot_modules::moderation::ModerationModule;

pub struct SetupCommand;

impl SetupCommand {
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

        let mut deny_perm = Permissions::READ_MESSAGES;
        deny_perm.insert(Permissions::SEND_MESSAGES);

        roles_perms.push(PermissionOverwrite {
            allow: Permissions::empty(),
            deny: deny_perm,
            kind: PermissionOverwriteType::Role(msg.guild_id.unwrap().0.into())
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
                name: String::from("mute-role"),
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

