use crate::database::models::{Server, NewServer};
use crate::database::schema::servers;
use diesel::associations::HasTable;
use diesel::{RunQueryDsl, TextExpressionMethods};
use crate::database::get_db_con;
use diesel::prelude::*;
use crate::database::schema::servers::columns::guildid;
use crate::config::DEFAULT_PREFIX;
use serenity::model::id::GuildId;

pub fn create_server(guild_id: String, enabled_modules: Vec<String>, disabled_cmds: Vec<String>) -> Server {
    let new_server = NewServer {
        guildid: guild_id,
        enabledmodules: enabled_modules,
        disabledcommands: disabled_cmds
    };

    diesel::insert_into(servers::table)
        .values(&new_server)
        .get_result(&get_db_con().get().expect("Pool!"))
        .expect("Error occurred while inserting new server")
}

pub fn get_server(guild_id: Option<GuildId>) -> Option<Server> {
    if guild_id.is_none() {
        return None;
    }

    let db = get_db_con().get().expect("Could not get db pool!");
    let g_id = guild_id.unwrap().to_string();
    let mut results: Vec<Server> = servers::dsl::servers.filter(guildid.like(g_id.to_owned()))
        .limit(1)
        .load::<Server>(&db)
        .expect("Could not load servers!");

    if results.len() == 0 {
        return Some(create_server(g_id, Vec::new(), Vec::new()));
    } else {
        return Some(results[0].clone())
    }
}