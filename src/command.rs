use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::Error;

pub const EMBED_REGULAR_COLOR: i32 = 714968;
pub const EMBED_ERROR_COLOR: i32 = 13632773;

pub enum CommandPerms {
    ServerOwner,
    Ban,
    Modules,
}

pub struct CommandConfig {
    pub name: String,
    pub values: Vec<String>
}

pub struct CommandArg {
    pub name: String,
    pub desc: Option<String>,
    pub optional: bool,
    pub next: Option<Box<CommandArg>>
}

pub trait Command {
    fn name(&self) -> String;
    fn desc(&self) -> String;
    fn enabled(&self) -> bool;
    fn use_in_dm(&self) -> bool;
    fn args(&self) -> Option<Vec<CommandArg>>;
    fn perms(&self) -> Option<Vec<CommandPerms>>;
    fn config(&self) -> Option<Vec<CommandConfig>>;
    fn exe(&self, ctx: &Context,  msg: &Message) -> Result<(), String>;
}


pub fn get_args(msg: Message) -> Vec<String> {
    let mut args: Vec<String> = msg.content.trim().split_whitespace().map(|a| a.to_string()).collect();
    args.remove(0);
    args
}

fn check_option(arg: &CommandArg, message: &str) -> Result<(), String> {
    if arg.name.contains("/") {
        let mut name = arg.name.to_owned();
        name.remove(0);
        name.remove(name.len() - 1);
        let options: Vec<&str> = name.split("/").collect();
        if !options.contains(&message) {
            return Err(format!("Invalid argument `{}`", message));
        }
    }

    Ok(())
}

pub fn parse_args(args: &Vec<CommandArg>, message_args: &Vec<String>) -> Result<Option<Vec<CommandArg>>, String> {
    let mut qualified_arg_routes: Vec<Vec<CommandArg>> = Vec::new();
    'main: for a in args.iter() {
        let mut depth = 0;
        let mut route: Vec<CommandArg> = Vec::new();

        if message_args.len() == 0 {
            return Ok(None);
        }

        // TODO: if arg name looks like that: <true/false> check for condition
        if !a.name.starts_with("<") && !a.name.ends_with(">") {
            if message_args.len() == 0 {
                continue
            }
            if a.name != message_args[0] {
                continue;
            }
        } else {
            check_option(&a, message_args[depth].as_str())?;
        }

        route.push(CommandArg{
            name: a.name.to_owned(),
            desc: a.desc.to_owned(),
            optional: a.optional,
            next: None
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
                check_option(&na, message_args[depth + 1].as_str())?;
            }

            route.push(CommandArg{
                name: na.name.to_owned(),
                desc: na.desc.to_owned(),
                optional: na.optional,
                next: None
            });

            depth += 1;
            next_arg = na.next.as_ref();
        }

        if route.len() == message_args.len() {
            return Ok(Some(route))
        }
    }

    Err(String::from("Invalid arguments!"))
}