use serenity::model::channel::{Message, Channel, ReactionType, PermissionOverwrite, PermissionOverwriteType};
use serenity::model::user::User;
use serenity::model::id::ChannelId;
use serenity::model::Permissions;
use serenity::prelude::Context;
use crate::database::models::{Server, SpecialEntityType, TempOperation};
use crate::database::schema::{servers, temp_operations};
use crate::database::schema::temp_operations::columns::{id, action_type};
use crate::diesel::{RunQueryDsl, BelongingToDsl, ExpressionMethods, QueryDsl, GroupedBy};
use crate::utils::db::{ServerInfo, ActionType, create_action, get_special_entity_by_type, create_temp_operation};
use crate::command::{Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR};
use crate::database::get_db_con;
use chrono::{Utc, Duration};
use std::thread;
use std::sync::Mutex;
use std::time::Duration as StdDuration;
use log::error;

pub struct SolvedTicketCommand;

impl SolvedTicketCommand {
    pub fn solve(&self, ctx: &Context, channel_id: ChannelId, user: &User, info: &ServerInfo) -> Result<(), String> {
        let ticket_category = match get_special_entity_by_type(info, SpecialEntityType::TicketsCategory) {
            Some(cat_id) => cat_id.entity_id,
            None => return Err(String::from("Tickets' category does not exist. Please use `!setup tickets`!"))
        };

        let err_msg = String::from("This is not a ticket!");
        let channel = if let Channel::Guild(ch) = ctx.cache.as_ref().read().channel(channel_id).unwrap() {
            match ch.read().category_id {
                Some(cat) => if cat.to_string() != ticket_category {
                    return Err(err_msg)
                } else {
                    ch.read().clone()
                }
                None => return Err(err_msg)
            }
        } else {
            return Err(err_msg)
        };

        let mut user_id = user.id;
        for p in channel.permission_overwrites.iter() {
            if let PermissionOverwriteType::Member(m) = p.kind {
                if p.deny == Permissions::SEND_MESSAGES {
                    return Err("Channel is already marked as solved!".to_string())
                }
                user_id = m;
            }
        }

        let mut perms = Permissions::READ_MESSAGES;
        perms.insert(Permissions::ADD_REACTIONS);
        let _ = channel_id.create_permission(&ctx.http, &PermissionOverwrite {
            allow: perms,
            deny: Permissions::SEND_MESSAGES,
            kind: PermissionOverwriteType::Member(user_id)
        });

        create_action(
            info,
            user_id.to_string(),
            Some(channel_id.to_string()),
            ActionType::SolvedTicket,
            format!("{} has been solved by {}.", channel.name, user.name)
        );
        create_temp_operation(info, channel_id.to_string(), Utc::now() + Duration::minutes(1), ActionType::SolvedTicket);

        let final_msg = channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Marked as solved!");
                e.description("Ticket will be removed after 1 hour! If you wish to reopen the ticket react with ❎.");
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });

        match final_msg {
            Ok(m) => {
                let _ = m.react(ctx.http.clone(), ReactionType::Unicode(String::from("❎")));
            },
            Err(_) => return Err("Could not create a message.".to_string())
        }

        Ok(())
    }
}

impl Command for SolvedTicketCommand {
    fn name(&self) -> String {
        String::from("solved")
    }

    fn desc(&self) -> String {
        String::from("Marks current ticket as solved")
    }

    fn use_in_dm(&self) -> bool {
        false
    }

    fn args(&self) -> Option<Vec<CommandArg>> {
        None
    }

    fn perms(&self) -> Option<Vec<String>> {
        None
    }

    fn config(&self) -> Option<Vec<CommandConfig>> {
        None
    }

    fn exe(&self, ctx: &Context, msg: &Message, info: &ServerInfo) -> Result<(), String> {
        self.solve(ctx, msg.channel_id, &msg.author, info)?;
        let _ = msg.delete(ctx.http.clone());
        Ok(())
    }

    fn init(&self, ctx: &Context) {
 let ctx = Mutex::new(ctx.clone());
        thread::spawn(move || {
            let db = get_db_con().get().expect("Could not get db pool!");
            loop {
                thread::sleep(StdDuration::from_secs(60));
                let servers = servers::dsl::servers
                    .load::<Server>(&db)
                    .expect("Could not load servers!");

                let tickets = TempOperation::belonging_to(&servers)
                    .filter(action_type.eq(ActionType::SolvedTicket as i32))
                    .load::<TempOperation>(&db)
                    .expect("Could not load temp operations")
                    .grouped_by(&servers);

                let data = servers.into_iter().zip(tickets).collect::<Vec<_>>();

                for v in data {
                    for t in v.1 {
                        if t.end_date < Utc::now().naive_utc() {
                            let channel_id = t.target_id.parse::<u64>().unwrap();
                            match &ctx.lock().unwrap().http.delete_channel(channel_id) {
                                Ok(_) => {/* send dm */},
                                Err(_) => error!("Could not close the ticket")
                            }
                            let _ = diesel::delete(temp_operations::table.filter(id.eq(t.id)))
                                .execute(&db);
                        }
                    }
                }
            }
        });
    }
}
