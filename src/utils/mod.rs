use serenity::model::channel::Message;

pub mod object_finding;
pub mod db;
pub mod perms;

pub fn check_if_dev(msg: &Message) -> bool {
    msg.author.id.to_string() == "246604909451935745"
}