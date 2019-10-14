mod config;
mod handler;
mod bot_modules;

use log::{info, error};
use serenity::Client;
use handler::*;

fn main() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        .apply()
        .expect("Could not configure the fern logger");

    info!("Starting...");

    let mut client = Client::new(&config::BOT_CONFIG.token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
