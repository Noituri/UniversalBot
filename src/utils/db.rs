use serenity::model::id::GuildId;
use crate::database::models::{Role, Server, NewRole, NewServer, NewDBCommand, DBCommand};
use crate::database::get_db_con;
use diesel::{RunQueryDsl, QueryDsl, BelongingToDsl, TextExpressionMethods};
use crate::database::schema::servers::columns::guildid;
use crate::database::schema::{servers, roles, commands};

pub fn create_db_server(
    guild_id: String,
    enabled_modules: Vec<String>,
    disabled_cmds: Vec<String>,
) -> Server {
    let new_server = NewServer {
        guildid: guild_id,
        enabledmodules: enabled_modules,
    };

    diesel::insert_into(servers::table)
        .values(&new_server)
        .get_result(&get_db_con().get().expect("Pool!"))
        .expect("Error occurred while inserting new server")
}

pub fn get_db_server(guild_id: Option<GuildId>) -> Option<Server> {
    if guild_id.is_none() {
        return None;
    }

    let db = get_db_con().get().expect("Could not get db pool!");
    let g_id = guild_id.unwrap().to_string();
    let results: Vec<Server> = servers::dsl::servers
        .filter(guildid.like(g_id.to_owned()))
        .limit(1)
        .load::<Server>(&db)
        .expect("Could not load servers!");

    if results.len() == 0 {
        return Some(create_db_server(g_id, Vec::new(), Vec::new()));
    } else {
        return Some(results[0].clone());
    }
}

pub fn create_db_role(server: &Server, role_id: String, perms: Vec<String>) -> Role {
    let new_role = NewRole {
        server_id: server.id,
        role_id: role_id,
        perms: perms,
    };

    diesel::insert_into(roles::table)
        .values(&new_role)
        .get_result(&get_db_con().get().expect("Could not get db pool!"))
        .expect("Error occurred while inserting new server")
}

pub fn get_db_role(guild_id: Option<GuildId>, role_id: String) -> Option<Role> {
    if guild_id.is_none() {
        return None;
    }

    let db = get_db_con().get().expect("Could not get db pool!");
    let g_id = guild_id.unwrap().to_string();
    let servers: Vec<Server> = servers::dsl::servers
        .filter(guildid.like(g_id.to_owned()))
        .limit(1)
        .load::<Server>(&db)
        .expect("Could not load servers!");

    let query = Role::belonging_to(&servers[0]).filter(roles::columns::role_id.like(&role_id)).first(&db);
    if let Ok(result) = query {
        return Some(result)
    } else {
        return Some(create_db_role(&servers[0], role_id, Vec::new()))
    }
}

pub fn create_db_command(server: &Server, cmd_name: String, channels: Vec<String>) -> DBCommand {
    let new_cmd = NewDBCommand {
        server_id: server.id,
        command_name: cmd_name,
        enabled_channels: channels
    };

    diesel::insert_into(commands::table)
        .values(&new_cmd)
        .get_result(&get_db_con().get().expect("Could not get db pool!"))
        .expect("Error occurred while inserting new server")
}

pub fn get_db_command(guild_id: Option<GuildId>, command_name: String) -> Option<DBCommand> {
    if guild_id.is_none() {
        return None;
    }

    let db = get_db_con().get().expect("Could not get db pool!");
    let g_id = guild_id.unwrap().to_string();
    let servers: Vec<Server> = servers::dsl::servers
        .filter(guildid.like(g_id.to_owned()))
        .limit(1)
        .load::<Server>(&db)
        .expect("Could not load servers!");

    let query = DBCommand::belonging_to(&servers[0]).filter(commands::columns::command_name.like(&command_name)).first(&db);
    if let Ok(result) = query {
        return Some(result)
    } else {
        return Some(create_db_command(&servers[0], command_name, Vec::new()))
    }
}