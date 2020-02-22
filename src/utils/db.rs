use serenity::model::id::GuildId;
use crate::database::models::{Role, Server, NewRole, NewServer, NewDBCommand, DBCommand, NewAction, NewTempBanMute};
use crate::database::get_db_con;
use diesel::{RunQueryDsl, QueryDsl, BelongingToDsl, TextExpressionMethods};
use crate::database::schema::servers::columns::guildid;
use crate::database::schema::{servers, roles, commands, actions, temp_bans_mutes};
use chrono::{DateTime, Utc};

pub struct ServerInfo {
    pub server: Option<Server>,
    pub commands: Option<Vec<DBCommand>>,
    pub roles: Option<Vec<Role>>
}

impl ServerInfo {
    pub fn new(guild_id: Option<GuildId>) -> ServerInfo {
        let server = get_db_server(guild_id);
        let mut commands = None;
        let mut roles = None;
        if let Some(s) = server.to_owned() {
            commands = get_db_commands(&s);
            roles = get_db_roles(&s);
        }

        ServerInfo {
            server,
            commands,
            roles
        }
    }
}

pub fn create_db_server(guild_id: String) -> Server {
    let new_server = NewServer {
        guildid: guild_id,
        enabledmodules: Vec::new(),
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
        return Some(create_db_server(g_id));
    } else {
        return Some(results[0].clone());
    }
}

pub fn create_db_role(server: &Server, role_id: String) -> Role {
    let new_role = NewRole {
        server_id: server.id,
        role_id: role_id,
        perms: Vec::new(),
    };

    diesel::insert_into(roles::table)
        .values(&new_role)
        .get_result(&get_db_con().get().expect("Could not get db pool!"))
        .expect("Error occurred while inserting new server")
}

pub fn get_db_roles(server: &Server) -> Option<Vec<Role>> {
    let db = get_db_con().get().expect("Could not get db pool!");

    let query = Role::belonging_to(server).load::<Role>(&db);
    if let Ok(result) = query {
        return Some(result)
    }

    None
}

pub fn get_db_role_by_id(info: &ServerInfo, role_id: String) -> Option<Role> {
    let server = match &info.server {
        Some(s) => s,
        None => return None
    };

    match &info.roles {
        Some(roles) => {
            for v in roles.iter() {
                if v.role_id == role_id {
                    return Some(v.clone())
                }
            }
        },
        None => {
            let db = get_db_con().get().expect("Could not get db pool!");
            let query = Role::belonging_to(server).filter(roles::role_id.like(&role_id)).first(&db);

            if let Ok(result) = query {
                return Some(result)
            }
        }
    }

    Some(create_db_role(&server, role_id))
}

pub fn create_db_command(server: &Server, cmd_name: String) -> DBCommand {
    let new_cmd = NewDBCommand {
        server_id: server.id,
        command_name: cmd_name,
        enabled_channels: Vec::new()
    };

    diesel::insert_into(commands::table)
        .values(&new_cmd)
        .get_result(&get_db_con().get().expect("Could not get db pool!"))
        .expect("Error occurred while inserting new server")
}

pub fn get_db_command_by_name(info: &ServerInfo, command_name: String) -> Option<DBCommand> {
    let server = match &info.server {
        Some(s) => s,
        None => return None
    };

    match &info.commands {
        Some(commands) => {
            for v in commands.iter() {
                if v.command_name == command_name {
                    return Some(v.clone())
                }
            }
        },
        None => {
            let db = get_db_con().get().expect("Could not get db pool!");
            let query = DBCommand::belonging_to(server).filter(commands::columns::command_name.like(&command_name)).first(&db);

            if let Ok(result) = query {
                return Some(result)
            }
        }
    }

    Some(create_db_command(server, command_name))
}

pub fn get_db_commands(server: &Server) -> Option<Vec<DBCommand>> {
    let db = get_db_con().get().expect("Could not get db pool!");
    let query = DBCommand::belonging_to(server).load::<DBCommand>(&db);

    if let Ok(result) = query {
        return Some(result)
    } else {
        None
    }
}

#[allow(dead_code)]
pub enum ActionType {
    Ban = 1,
    UnBan = 2,
    Kick = 3,
    Mute = 4
}

pub fn create_action(info: &ServerInfo, issuer: String, target: Option<String>, action_type: ActionType, message: String) {
    let new_action = NewAction {
        server_id: info.server.clone().unwrap().id,
        action_type: action_type as i32,
        creation_date: Utc::now().naive_utc(),
        issuer,
        target,
        message
    };

    diesel::insert_into(actions::table)
        .values(&new_action)
        .execute(&get_db_con().get().expect("Could not get db pool!"))
        .expect("Error occurred while inserting new server");
}

pub fn create_temp_ban_mute(info: &ServerInfo, user_id: String, end_date: DateTime<Utc>, action_type: ActionType) {
    let new_entry = NewTempBanMute {
        server_id: info.server.clone().unwrap().id,
        action_type: action_type as i32,
        end_date: end_date.naive_utc(),
        user_id,
    };

    diesel::insert_into(temp_bans_mutes::table)
        .values(&new_entry)
        .execute(&get_db_con().get().expect("Could not get db pool!"))
        .expect("Error occurred while inserting new server");
}