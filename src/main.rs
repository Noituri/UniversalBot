#[macro_use]
extern crate diesel;

mod bot_modules;
mod command;
mod config;
mod database;
mod handler;
#[cfg(test)]
mod tests;
mod utils;

use bot_modules::*;
use handler::*;
use log::{error, info};
use serenity::Client;

fn main() {
    pretty_env_logger::init();
    info!("Initializing database...");
    {
        let _ = database::get_db_con();
    }

    info!("Starting bot...");
    let mut client = Client::new(&config::BOT_CONFIG.token, Handler).expect("Err creating client");
    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
