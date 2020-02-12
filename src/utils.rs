use crate::database::models::{Server, NewServer, Role};
use crate::database::schema::servers;
use diesel::{RunQueryDsl, TextExpressionMethods};
use crate::database::get_db_con;
use diesel::prelude::*;
use crate::database::schema::servers::columns::guildid;
use serenity::model::id::{GuildId};
use serenity::prelude::Context;
use serenity::model::prelude::Message;

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
    let results: Vec<Server> = servers::dsl::servers.filter(guildid.like(g_id.to_owned()))
        .limit(1)
        .load::<Server>(&db)
        .expect("Could not load servers!");

    if results.len() == 0 {
        return Some(create_server(g_id, Vec::new(), Vec::new()));
    } else {
        return Some(results[0].clone())
    }
}

pub fn has_perms(ctx: &Context, msg: &Message, server: Server, perms: &Option<Vec<String>>) -> bool {
    let guild = match msg.guild(ctx.clone().cache) {
        Some(g) => g,
        None => return true
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