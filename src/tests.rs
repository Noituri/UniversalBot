use super::*;
use crate::command::{get_args, parse_args, CommandArg};
use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use serenity::model::channel::{Message, MessageType};
use serenity::model::user::User;

#[test]
fn test_parse_args() {
    let c_args = vec![CommandArg {
        name: "name".to_string(),
        desc: None,
        next: Some(Box::new(CommandArg {
            name: "<ok/no>".to_string(),
            desc: None,
            next: None,
            option: None,
        })),
        option: None,
    }];

    assert_eq!(
        parse_args(&c_args, &vec!["name".to_string(), "ok".to_string()])
            .unwrap()
            .unwrap()
            .len(),
        2
    );

    assert!(parse_args(&c_args, &vec!["owo".to_string(), "ok".to_string()]).is_err());

    assert!(parse_args(&c_args, &vec![]).unwrap().is_none());

    assert!(parse_args(&c_args, &vec!["name".to_string(), "not_ok".to_string()]).is_err());
}
