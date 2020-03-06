use serenity::model::id::GuildId;
use crate::database::models::{Role, Server, NewRole, NewServer, NewDBCommand, DBCommand, NewAction, NewTempBanMute, NewSpecialEntity, SpecialEntityType, SpecialEntity, Action};
use crate::database::get_db_con;
use diesel::{RunQueryDsl, QueryDsl, BelongingToDsl, TextExpressionMethods, ExpressionMethods, BoolExpressionMethods, Expression};
use crate::database::schema::servers::columns::guildid;
use crate::database::schema::{servers, roles, commands, actions, temp_bans_mutes, special_entities};
use chrono::{DateTime, Utc};
use crate::database::schema::actions::columns::{action_type, target, creation_date};

pub struct ServerInfo {
    pub server: Option<Server>,
    pub commands: Option<Vec<DBCommand>>,
    pub roles: Option<Vec<Role>>,
    pub special_entities: Option<Vec<SpecialEntity>>
}

impl ServerInfo {
    pub fn new(guild_id: Option<GuildId>) -> ServerInfo {
        let server = get_db_server(guild_id);
        let mut commands = None;
        let mut roles = None;
        let mut special_entities = None;
        if let Some(s) = server.to_owned() {
            commands = get_db_commands(&s);
            roles = get_db_roles(&s);
            special_entities = get_special_entities(&s);
        }

        ServerInfo {
            server,
            commands,
            roles,
            special_entities
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
        .expect("Error occurred while inserting new role")
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
        .expect("Error occurred while inserting new command")
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
#[derive(Copy, Clone)]
pub enum ActionType {
    Ban = 1,
    UnBan = 2,
    Kick = 3,
    Mute = 4,
    UnMute = 5,
    Warn = 6,
    ReducedWarn = 7,
    ChannelLock = 8,
    ChannelUnLock = 9,
}

pub fn get_user_warn_lvl(info: &ServerInfo, user_id: &str) -> i64 {
    let server = match &info.server {
        Some(s) => s,
        None => return 0
    };

    let db = get_db_con().get().expect("Could not get db pool!");
    let warns: i64 = Action::belonging_to(server)
        .filter(action_type.eq(ActionType::Warn as i32))
        .filter(target.like(user_id))
        .count()
        .get_result(&db)
        .expect("Could not load warn actions");

    let reduced_warns: i64 = Action::belonging_to(server)
        .filter(action_type.eq(ActionType::ReducedWarn as i32))
        .filter(target.like(user_id))
        .count()
        .get_result(&db)
        .expect("Could not load reduced actions");

    warns - reduced_warns
}

pub fn get_actions_by_kind(info: &ServerInfo, user_id: String, kinds: Vec<ActionType>,) -> Option<Vec<Action>> {
    let server = match &info.server {
        Some(s) => s,
        None => return None
    };
    if kinds.len() == 0 {
        return None
    }

    let db = get_db_con().get().expect("Could not get db pool!");
    let mut query = format!(r#"action_type = {}"#, kinds[0] as i32);
    if kinds.len() > 1 {
        for k in kinds[1..].to_vec() {
            query.push_str(&format!(r#"OR action_type = {}"#, k as i32));
        }
    }

    Some(diesel::sql_query(format!(r#"SELECT * FROM actions
        WHERE server_id = {}
        AND
        target = '{}'
        AND
        ({})
        ORDER BY creation_date DESC;"#, server.id, user_id, query))
        .load(&db)
        .expect("Could not load actions"))
}

pub fn create_action(info: &ServerInfo, issuer: String, target_id: Option<String>, action_kind: ActionType, message: String) {
    let new_action = NewAction {
        server_id: info.server.clone().unwrap().id,
        action_type: action_kind as i32,
        creation_date: Utc::now().naive_utc(),
        target: target_id,
        issuer,
        message
    };

    diesel::insert_into(actions::table)
        .values(&new_action)
        .execute(&get_db_con().get().expect("Could not get db pool!"))
        .expect("Error occurred while inserting new action");
}

pub fn create_temp_ban_mute(info: &ServerInfo, user_id: String, end_date: DateTime<Utc>, action_kind: ActionType) {
    let new_entry = NewTempBanMute {
        server_id: info.server.clone().unwrap().id,
        action_type: action_kind as i32,
        end_date: end_date.naive_utc(),
        user_id,
    };

    diesel::insert_into(temp_bans_mutes::table)
        .values(&new_entry)
        .execute(&get_db_con().get().expect("Could not get db pool!"))
        .expect("Error occurred while inserting new temp ban/mute");
}

pub fn get_special_entities(server: &Server) -> Option<Vec<SpecialEntity>> {
    let db = get_db_con().get().expect("Could not get db pool!");
    let query = SpecialEntity::belonging_to(server).load::<SpecialEntity>(&db);

    if let Ok(result) = query {
        return Some(result)
    } else {
        None
    }
}

pub fn get_special_entity_by_type(info: &ServerInfo, kind: SpecialEntityType) -> Option<SpecialEntity> {
    let server = match &info.server {
        Some(s) => s,
        None => return None
    };

    match &info.special_entities {
        Some(entities) => {
            for v in entities.iter() {
                if v.entity_type == kind as i32 {
                    return Some(v.clone())
                }
            }
        },
        None => {
            let db = get_db_con().get().expect("Could not get db pool!");
            let query = SpecialEntity::belonging_to(server)
                .filter(special_entities::columns::entity_type.eq(kind as i32)).first(&db);

            if let Ok(result) = query {
                return Some(result)
            }
        }
    }

    None
}

pub fn create_special_entity(info: &ServerInfo, entity_id: String, kind: SpecialEntityType) {
    let new_entity = NewSpecialEntity {
        server_id: info.server.clone().unwrap().id,
        entity_type: kind as i32,
        entity_id: entity_id.to_owned()
    };

    let db = &get_db_con().get().expect("Could not get db pool!");
    match get_special_entity_by_type(info, kind) {
        Some(e) => diesel::update(special_entities::dsl::special_entities.find(e.id))
            .set(special_entities::entity_id.eq(&entity_id))
            .execute(db)
            .expect("Could not update the server!"),
        None => diesel::insert_into(special_entities::table)
            .values(&new_entity)
            .execute(db)
            .expect("Error occurred while inserting new special entity")
    };
}