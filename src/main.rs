mod config;
mod handler;
mod bot_modules;
mod command;

use log::{info, error};
use serenity::Client;
use handler::*;
use bot_modules::bot_modules::*;

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
    println!("MODULE: {:#?}", get_modules()[0].name());
    let mut client = Client::new(&config::BOT_CONFIG.token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
