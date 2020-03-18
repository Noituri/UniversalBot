use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::bot_modules::{get_modules, PROTECTED_MODULES};
use crate::utils::db::ServerInfo;
use crate::utils::get_time;

pub const EMBED_REGULAR_COLOR: i32 = 714968;
pub const EMBED_QUESTION_COLOR: i32 = 16772147;
pub const EMBED_ERROR_COLOR: i32 = 13632773;

pub struct CommandConfig {
    pub name: String,
    pub values: Vec<String>,
}

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum ArgOption {
    Numeric,
    Text,
    Boolean,
    Any,
    Role,
    Channel,
    User,
    UserId,
    Time
}

pub struct CommandArg {
    pub name: String,
    pub desc: Option<String>,
    pub option: Option<ArgOption>,
    pub next: Option<Box<CommandArg>>,
}

pub trait Command {
    fn name(&self) -> String;
    fn desc(&self) -> String;
    fn use_in_dm(&self) -> bool;
    fn args(&self) -> Option<Vec<CommandArg>>;
    fn perms(&self) -> Option<Vec<String>>;
    fn config(&self) -> Option<Vec<CommandConfig>>;
    fn exe(&self, ctx: &Context, msg: &Message, server: &ServerInfo) -> Result<(), String>;
    fn init(&self, _ctx: &Context) {}
}

impl dyn Command {
    pub fn disabled(&self, info: &ServerInfo, channel_id: String) -> bool {
        let mut exists = false;
        if let Some(commands) = &info.disabled_commands {
            for v in commands.iter() {
                if v.command_name == self.name() && v.disabled_channels.contains(&channel_id) {
                    exists = true;
                    break
                }
            }
        }
        println!("name: {} -> {}", self.name(), exists && !is_command_protected(&self.name()).unwrap());
        exists && !is_command_protected(&self.name()).unwrap()
    }
}

pub fn get_args(msg: Message, include_cmd: bool) -> Vec<String> {
    let mut args: Vec<String> = msg
        .content
        .trim()
        .split_whitespace()
        .map(|a| a.to_string())
        .collect();

    if !include_cmd {
        if msg.content.starts_with("<@") && args.len() != 0 {
            args.remove(0);
        }
        if args.len() != 0 {
            args.remove(0);
        }
    }

    args
}

// if Ok(true) it will skip this route
fn check_option(arg: &CommandArg, message: &str) -> Result<bool, String> {
    if let Some(op) = &arg.option {
        match op {
            ArgOption::Numeric => {
                if message.parse::<f64>().is_err() {
                    return Ok(true);
                }
            }
            ArgOption::Text => {
                if message.parse::<f64>().is_ok() {
                    return Ok(true);
                }
            }
            ArgOption::Boolean => match message {
                "yes" | "no" | "true" | "false" => {}
                _ => return Ok(true),
            },
            ArgOption::Role => {
                if message.starts_with("<@&") && message.ends_with(">") {
                    if message.len() != 22 {
                        return Ok(true);
                    }
                    if message[3..message.len()-1].parse::<u64>().is_err() {
                        return Ok(true);
                    }
                }
            },
            ArgOption::Channel => {
                if message.starts_with("<#") && message.ends_with(">") {
                    if message.len() != 21 {
                        return Ok(true);
                    }
                    if message[2..message.len()-1].parse::<u64>().is_err() {
                        return Ok(true);
                    }
                }
            },
            ArgOption::User | ArgOption::UserId => {
                if message.starts_with("<@!") && message.ends_with(">") {
                    if message.len() != 22 {
                        return Ok(true);
                    }
                    if message[3..message.len()-1].parse::<u64>().is_err() {
                        return Ok(true);
                    }
                } else if message.starts_with("<@") && message.ends_with(">") {
                    if message.len() != 21 {
                        return Ok(true);
                    }
                    if message[2..message.len()-1].parse::<u64>().is_err() {
                        return Ok(true);
                    }
                } else if op.to_owned() == ArgOption::UserId {
                    if message.len() != 18 {
                        return Ok(true);
                    }
                    if message.parse::<u64>().is_err() {
                        return Ok(true);
                    }
                }
            },
            ArgOption::Time => {
                return Ok(get_time(message).is_err());
            },
            ArgOption::Any => {}
        }
    }

    if arg.name.contains("/") {
        let mut name = arg.name.to_owned();
        name.remove(0);
        name.remove(name.len() - 1);
        let options: Vec<&str> = name.split("/").collect();
        if !options.contains(&message) {
            return Err(format!("Invalid argument `{}`", message));
        }
    }

    Ok(false)
}

impl CommandArg {
    fn is_arg_variable(&self) -> bool {
        self.name.starts_with("<") && self.name.ends_with(">") ||
            self.name.starts_with("[") && self.name.ends_with("]")
    }

    fn is_optional(&self) -> bool {
        self.name.starts_with("[") && self.name.ends_with("]")
    }

    fn accepts_more(&self) -> bool {
        self.name.ends_with("...>") || self.name.ends_with("...]")
    }
}

pub fn parse_args(
    args: &Vec<CommandArg>,
    message_args: &Vec<String>,
) -> Result<Option<Vec<CommandArg>>, String> {
    'main: for a in args.iter() {
        let mut depth = 0;
        let mut route: Vec<CommandArg> = Vec::new();

        if message_args.len() == 0 {
            return Ok(None);
        }

        if !a.is_arg_variable() {
            if a.name != message_args[0] {
                continue;
            }
        } else {
            if check_option(&a, message_args[depth].as_str())? {
                continue;
            }
        }

        route.push(CommandArg {
            name: a.name.to_owned(),
            desc: a.desc.to_owned(),
            option: a.option.clone(),
            next: None,
        });

        let mut next_arg = a.next.as_ref();
        'nextArg: while next_arg.is_some() {
            let na = next_arg.unwrap();
            if depth >= message_args.len() - 1 {
                if na.is_optional() {
                    break;
                }
                continue 'main;
            }

            if !na.is_arg_variable() {
                if na.name != message_args[depth + 1] {
                    continue 'main;
                }
            } else {
                if check_option(&na, message_args[depth + 1].as_str())? {
                    if na.is_optional() {
                        next_arg = na.next.as_ref();
                        continue 'nextArg;
                    } else {
                        continue 'main;
                    }
                }
            }

            route.push(CommandArg {
                name: na.name.to_owned(),
                desc: na.desc.to_owned(),
                option: na.option.clone(),
                next: None,
            });

            depth += 1;
            next_arg = na.next.as_ref();
        }

        if route.len() == message_args.len() {
            return Ok(Some(route));
        } else if route.len() < message_args.len() {
            if let Some(ma) = route.last() {
                if ma.accepts_more() {
                    return Ok(Some(route));
                }
            }
        }
    }

    Err(String::from("Invalid arguments!"))
}

pub fn find_command(name: &str, info: &ServerInfo) -> Result<Box<dyn Command>, String> {
    for m in get_modules() {
        for c in m.commands() {
            if c.name() == name {
                if m.enabled(info) {
                    return Ok(c)
                } else {
                    return Err("Command is in disabled module!".to_string())
                }
            }
        }
    }
    Err("Command does not exist!".to_string())
}

pub fn is_command_protected(name: &str) -> Result<bool, String> {
    for m in get_modules() {
        for c in m.commands() {
            if c.name() == name {
                return Ok(PROTECTED_MODULES.contains(&m.name().as_str()))
            }
        }
    }
    Err("Command does not exist!".to_string())
}
