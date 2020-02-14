use crate::database::get_db_con;
use crate::database::models::{NewServer, Role, Server, NewRole};
use crate::database::schema::servers;
use crate::database::schema::servers::columns::guildid;
use diesel::prelude::*;
use diesel::{RunQueryDsl, TextExpressionMethods};
use serenity::model::id::GuildId;
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use serenity::model::guild;
use crate::handler::{State, FindsAwaitingAnswer, FindType};
use crate::handler::STATE;
use std::sync::MutexGuard;
use crate::database::schema::roles;
use diesel::associations::HasTable;
use crate::command::EMBED_QUESTION_COLOR;

pub fn create_server(
    guild_id: String,
    enabled_modules: Vec<String>,
    disabled_cmds: Vec<String>,
) -> Server {
    let new_server = NewServer {
        guildid: guild_id,
        enabledmodules: enabled_modules,
        disabledcommands: disabled_cmds,
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
    let results: Vec<Server> = servers::dsl::servers
        .filter(guildid.like(g_id.to_owned()))
        .limit(1)
        .load::<Server>(&db)
        .expect("Could not load servers!");

    if results.len() == 0 {
        return Some(create_server(g_id, Vec::new(), Vec::new()));
    } else {
        return Some(results[0].clone());
    }
}

pub fn create_role(server: &Server, role_id: String, perms: Vec<String>) -> Role {
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

pub fn get_role(guild_id: Option<GuildId>, role_id: String) -> Option<Role> {
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
        return Some(create_role(&servers[0], role_id, Vec::new()))
    }
}

pub fn has_perms(
    ctx: &Context,
    msg: &Message,
    server: Server,
    perms: &Option<Vec<String>>,
) -> bool {
    let guild = match msg.guild(ctx.clone().cache) {
        Some(g) => g,
        None => return true,
    };
    let guild = guild.read();
    let is_owner = msg.author.id == guild.owner_id;
    let is_admin = guild.member_permissions(msg.author.id).administrator();

    if perms.is_some() && !is_owner && !is_admin {
        let db = get_db_con().get().expect("Could not get db pool!");
        let server_roles: Vec<Role> = Role::belonging_to(&server)
            .load::<Role>(&db)
            .expect("Could not get roles from guild");
        let mut has_perms = false;

        if !server_roles.is_empty() {
            let current_member = guild.member(ctx.http.clone(), msg.author.id).unwrap();
            for v in current_member.roles.iter() {
                let r_id = v.to_string();
                'outer: for sr in server_roles.iter() {
                    if r_id == sr.role_id {
                        for cp in perms.clone().unwrap().iter() {
                            if !sr.perms.contains(cp) {
                                break 'outer;
                            }
                        }
                        has_perms = true;
                    }
                }
            }
        }

        return has_perms;
    }

    true
}

pub fn check_if_dev(msg: &Message) -> bool {
    msg.author.id.to_string() == "246604909451935745 "
}

pub fn find_role(ctx: &Context, msg: &Message, g_roles: Vec<guild::Role>, find_text: &str) -> Result<u64, String> {
    let guild = if let Some(g) = msg.guild_id {
        g
    } else {
        return Err("Could not retrieve guild info!".to_string())
    };

    let mut matched_roles: Vec<(u64, String)> = Vec::new();
    for (i, v) in g_roles.iter().enumerate() {
        if v.name.to_lowercase().contains(&find_text.to_lowercase()) {
            matched_roles.push((v.id.0, format!("**{}.** {}\n", matched_roles.len()+1, v.name)))
        }
    }

    match matched_roles.len() {
        0 => return Err("Could not find requested role!".to_string()),
        1 => return Ok(matched_roles[0].0),
        l if l > 15 => return Err("Too many results. Please be more specific.".to_string()),
        _ => {
            let mut description = String::new();
            matched_roles.iter().for_each(|r| description.push_str(&r.1));
            {
                let mut state = STATE.lock().unwrap();
                state.role_finds_awaiting.push(FindsAwaitingAnswer{
                    find_type: FindType::Role,
                    who: msg.author.id.0,
                    when: 0,
                    finds: matched_roles,
                    replace_text: find_text.to_owned(),
                    msg_content: msg.content.to_owned()
                })
            }

            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("Which role did you have in mind?");
                    e.color(EMBED_QUESTION_COLOR);
                    e.description(description);
                    e.footer(|f| {
                        f.text("Respond with number corresponding to the role.");
                        f
                    });
                    e
                });
                m
            });
        }
    }
    Ok(0)
}

pub fn get_role_from_id(ctx: &Context, msg: &Message, id: String) -> Result<Option<guild::Role>, String> {
    let mut tmp_id = id;
    if msg.mention_roles.len() != 0 {
        tmp_id = msg.mention_roles[0].to_string();
    }
    let g_roles = if let Ok(guildRoles) = ctx.http.get_guild_roles(msg.guild_id.unwrap().0) {
        guildRoles
    } else {
        return Err("Could not retrieve guild roles!".to_string())
    };

    for v in g_roles.iter() {
        if &v.id.to_string() == &tmp_id{
            return Ok(Some(v.clone()))
        }
    }

    let found_role = find_role(ctx, msg, g_roles,&tmp_id)?;
    if found_role != 0 {
        return get_role_from_id(ctx, msg, found_role.to_string())
    };
    Ok(None)
}
