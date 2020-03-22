#[macro_use]
extern crate diesel;

mod bot_modules;
mod command;
mod config;
mod database;
mod handler;
mod utils;

#[cfg(test)]
mod tests;

use bot_modules::*;
use handler::*;
use log::{error, info};
use serenity::Client;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    info!("Initializing database...");
    {
        let _ = database::get_db_con();
    }

    info!("Starting bot...");
    let mut client = Client::new(&config::TOKEN, Handler).expect("Err creating client");
    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
