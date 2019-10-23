use super::*;
use serenity::model::channel::{Message, MessageType};
use serenity::model::user::User;
use crate::command::{get_args, CommandArg, parse_args};
use chrono::{DateTime, Utc, FixedOffset, TimeZone};

#[test]
fn test_parse_args() {
    let c_args = vec![
        CommandArg{
            name: "name".to_string(),
            desc: None,
            optional: false,
            next: Some(
                Box::new(
                    CommandArg{
                        name: "<ok/no>".to_string(),
                        desc: None,
                        optional: false,
                        next: None
                    }
                )
            )
        }
    ];

    assert_eq!(parse_args(&c_args, &vec![
        "name".to_string(),
        "ok".to_string()
    ]).unwrap().unwrap().len(), 2);

    assert!(parse_args(&c_args, &vec![
        "owo".to_string(),
        "ok".to_string()
    ]).is_err());

    assert!(parse_args(&c_args, &vec![]).unwrap().is_none());

    assert!(parse_args(&c_args, &vec![
        "name".to_string(),
        "not_ok".to_string()
    ]).is_err());
}