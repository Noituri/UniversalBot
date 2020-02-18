use serenity::model::channel::Message;

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

pub fn get_time(mut time_msg: &str, mut default_format: TimeFormat) -> Result<i32, String> {
    time_msg = time_msg.trim();
    if time_msg.is_empty() {
        return Err("Provided `time` is empty!".to_string())
    }

    let time = match time_msg.parse::<i32>() {
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


                match time_msg[..time_msg.len()-1].parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => return Err("`time` value is not an integer!".to_string())
                }
            } else {
                return Err("Invalid `time` has been provided!".to_string())
            }
        }
    };

    Ok(time)
}