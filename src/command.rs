use crate::database::models::Server;
use serenity::model::channel::Message;
use serenity::prelude::Context;

pub const EMBED_REGULAR_COLOR: i32 = 714968;
pub const EMBED_QUESTION_COLOR: i32 = 16772147;
pub const EMBED_ERROR_COLOR: i32 = 13632773;

pub struct CommandConfig {
    pub name: String,
    pub values: Vec<String>,
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum ArgOption {
    Numeric,
    Text,
    Boolean,
    Any,
    Role
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
    fn enabled(&self) -> bool;
    fn use_in_dm(&self) -> bool;
    fn args(&self) -> Option<Vec<CommandArg>>;
    fn perms(&self) -> Option<Vec<String>>;
    fn config(&self) -> Option<Vec<CommandConfig>>;
    fn exe(&self, ctx: &Context, msg: &Message, server: Option<Server>) -> Result<(), String>;
}

pub fn get_args(msg: Message) -> Vec<String> {
    let mut args: Vec<String> = msg
        .content
        .trim()
        .split_whitespace()
        .map(|a| a.to_string())
        .collect();
    if msg.content.starts_with("<@") && args.len() != 0 {
        args.remove(0);
    }
    if args.len() != 0 {
        args.remove(0);
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
                if message.starts_with("<@&") || message.ends_with(">") {
                    if message.len() != 22 {
                        return Ok(true);
                    }
                    if message[3..message.len()-1].parse::<f64>().is_err() {
                        return Ok(true);
                    }
                }
            }
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

        if !a.name.starts_with("<") && !a.name.ends_with(">") {
            if message_args.len() == 0 {
                continue;
            }
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
        while next_arg.is_some() {
            if depth >= message_args.len() - 1 {
                continue 'main;
            }
            let na = next_arg.unwrap();

            if !na.name.starts_with("<") && !na.name.ends_with(">") {
                if na.name != message_args[depth + 1] {
                    continue 'main;
                }
            } else {
                if check_option(&na, message_args[depth + 1].as_str())? {
                    continue 'main;
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
                if ma.name.ends_with("...>") {
                    return Ok(Some(route));
                }
            }
        }
    }

    Err(String::from("Invalid arguments!"))
}
