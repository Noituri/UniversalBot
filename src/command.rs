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
    fn args(&self) -> Option<Vec<CommandArg>>;
    fn perms(&self) -> Option<Vec<CommandPerms>>;
    fn config(&self) -> Option<Vec<CommandConfig>>;
    fn exe(&self, ctx: &Context,  msg: &Message) -> Result<Message, Error>;
}


pub fn get_args(msg: Message) -> Vec<String> {
    let mut args: Vec<String> = msg.content.trim().split_whitespace().map(|a| a.to_string()).collect();
    args.remove(0);
    args
}


pub fn parse_args(args: Vec<CommandArg>, message_args: Vec<String>) -> Result<Option<Vec<CommandArg>>, ()> {
    let mut qualified_arg_routes: Vec<Vec<CommandArg>> = Vec::new();
    'main: for a in args.iter() {
        let mut depth = 0;
        let mut route: Vec<CommandArg> = Vec::new();

        if message_args.len() == 0 {
            return Ok(None);
        }

        if !a.name.starts_with("<") && !a.name.ends_with(">") {
            if message_args.len() == 0 {
                continue
            }
            if a.name != message_args[0] {
                continue;
            }
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

    Err(())
}