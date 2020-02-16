use crate::database::models::{Role, Server};
use crate::database::get_db_con;
use serenity::prelude::Context;
use serenity::model::channel::Message;
use crate::config::DEV_MODULE;
use crate::bot_modules::get_modules;
use diesel::{RunQueryDsl, QueryDsl, BelongingToDsl};

pub fn get_module_perms(module_name: &str) -> Option<Vec<String>> {
    for m in get_modules().iter() {
        if m.name() == module_name {
            let mut perms = Vec::new();
            for c in m.commands().iter() {
                if let Some(perm) = c.perms() {
                    perms.extend(perm.iter().cloned());
                }
            }

            return Some(perms)
        }
    }
    None
}

pub fn perms_exists(perms: &Vec<String>) -> bool {
    let mut exists = true;
    let mut all_perms: Vec<String> = Vec::new();

    for m in get_modules().iter() {
        if m.name() == DEV_MODULE {
            continue
        }

        for c in m.commands().iter() {
            if let Some(perm) = c.perms() {
                all_perms.extend(perm.iter().cloned())
            }
        }
    }

    perms.iter().for_each(|p| {
        if !all_perms.contains(p) {
            exists = false;
        }
    });

    exists
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