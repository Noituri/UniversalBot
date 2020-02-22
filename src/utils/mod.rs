use serenity::model::channel::Message;
use chrono::{DateTime, Utc, Duration};

pub mod object_finding;
pub mod db;
pub mod perms;

pub fn check_if_dev(msg: &Message) -> bool {
    msg.author.id.to_string() == "246604909451935745"
}

pub fn get_time(mut time_msg: &str) -> Result<DateTime<Utc>, String> {
    time_msg = time_msg.trim();
    if time_msg.is_empty() {
        return Err("Provided `time` is empty!".to_string())
    }

    if time_msg.len() > 1 {
        let time = match time_msg[..time_msg.len()-1].parse::<i64>() {
            Ok(num) => num,
            Err(_) => return Err("`time` value is not an integer!".to_string())
        };

        let last_char = time_msg.chars().last().unwrap();
        return match last_char {
            's' => Ok(Utc::now() + Duration::seconds(time)),
            'm' => Ok(Utc::now() + Duration::minutes(time)),
            'h' => Ok(Utc::now() + Duration::hours(time)),
            'd' => Ok(Utc::now() + Duration::days(time)),
            _ => {
                return Err("Invalid `time` has been provided!".to_string())
            }
        }


    } else {
        return Err("Invalid `time` has been provided!".to_string())
    }
}