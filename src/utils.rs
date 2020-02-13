use crate::database::get_db_con;
use crate::database::models::{NewServer, Role, Server};
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
                    if r_id == sr.roleid {
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

pub fn find_role(ctx: &Context, msg: &Message, find_text_range: (usize, usize)) -> Result<u64, String> {
    let guild = if let Some(g) = msg.guild_id {
        g
    } else {
        return Err("Could not retrieve guild info!".to_string())
    };

    let roles = if let Ok(guild_roles) = ctx.http.get_guild_roles(*guild.as_u64()) {
        guild_roles
    } else {
        return Err("Could not retrieve roles from the guild!".to_string())
    };

    let mut matched_roles: Vec<(u64, String)> = Vec::new();
    let find_text = &msg.content[find_text_range.0 .. find_text_range.1];
    for (i, v) in roles.iter().enumerate() {
        if v.name.contains(find_text) {
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
                    replace_range: find_text_range,
                    msg_content: msg.content.to_owned()
                })
            }

            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("Which role did you have in mind?");
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
