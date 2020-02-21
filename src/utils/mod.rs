use serenity::model::channel::Message;
use chrono::{DateTime, Utc, Duration};

pub mod object_finding;
pub mod db;
pub mod perms;

pub fn check_if_dev(msg: &Message) -> bool {
    msg.author.id.to_string() == "246604909451935745"
}

pub enum TimeFormat {
    Seconds,
    Minutes,
    Hours,
    Days
}

pub fn get_time_string(mut time_msg: &str, format: TimeFormat) -> String {
    time_msg = time_msg.trim();

    return match time_msg.chars().last().unwrap() {
        's' | 'm' | 'h' | 'd' =>  time_msg.to_owned(),
        _ => {
            let suffix = match format {
                TimeFormat::Seconds => "s",
                TimeFormat::Minutes => "m",
                TimeFormat::Hours => "h",
                TimeFormat::Days => "d",
            };

            format!("{}{}", time_msg, suffix)
        }
    }
}

pub fn get_time(mut time_msg: &str, mut default_format: TimeFormat) -> Result<DateTime<Utc>, String> {
    time_msg = time_msg.trim();
    if time_msg.is_empty() {
        return Err("Provided `time` is empty!".to_string())
    }

    let time = match time_msg.parse::<i64>() {
        Ok(num) => num,
        Err(_) => {
            if time_msg.len() > 1 {
                let last_char = time_msg.chars().last().unwrap();
                match last_char {
                    's' => default_format = TimeFormat::Seconds,
                    'm' => default_format = TimeFormat::Minutes,
                    'h' => default_format = TimeFormat::Hours,
                    'd' => default_format = TimeFormat::Days,
                    _ => {
                        return Err("Invalid `time` has been provided!".to_string())
                    }
                }


                match time_msg[..time_msg.len()-1].parse::<i64>() {
                    Ok(num) => num,
                    Err(_) => return Err("`time` value is not an integer!".to_string())
                }
            } else {
                return Err("Invalid `time` has been provided!".to_string())
            }
        }
    };

    return match default_format {
        TimeFormat::Seconds => Ok(Utc::now() + Duration::seconds(time)),
        TimeFormat::Minutes => Ok(Utc::now() + Duration::minutes(time)),
        TimeFormat::Hours => Ok(Utc::now() + Duration::hours(time)),
        TimeFormat::Days => Ok(Utc::now() + Duration::days(time))
    }
}