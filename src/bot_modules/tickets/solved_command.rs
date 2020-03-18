use serenity::model::channel::{Message, Channel, ReactionType, PermissionOverwrite, PermissionOverwriteType};
use serenity::model::Permissions;
use serenity::prelude::Context;
use crate::database::models::SpecialEntityType;
use crate::utils::db::{ServerInfo, ActionType, create_action, get_special_entity_by_type, create_temp_operation};
use crate::command::{Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR};
use chrono::{Utc, Duration};

pub struct SolvedTicketCommand;

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
        let ticket_category = match get_special_entity_by_type(info, SpecialEntityType::TicketsCategory) {
            Some(id) => id.entity_id,
            None => return Err(String::from("Tickets' category does not exist. Please use `!setup tickets`!"))
        };

        let err_msg = String::from("This is not a ticket!");
        let channel = if let Channel::Guild(ch) = msg.channel(&ctx.cache).unwrap() {
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

        let mut user_id = msg.author.id;
        for p in channel.permission_overwrites.iter() {
            if let PermissionOverwriteType::Member(m) = p.kind {
                user_id = m;
            }
        }

        let mut perms = Permissions::READ_MESSAGES;
        perms.insert(Permissions::ADD_REACTIONS);
        let _ = msg.channel_id.create_permission(&ctx.http, &PermissionOverwrite {
            allow: perms,
            deny: Permissions::SEND_MESSAGES,
            kind: PermissionOverwriteType::Member(user_id)
        });

        create_action(
            info,
            msg.author.id.to_string(),
            Some(msg.channel_id.to_string()),
            ActionType::SolvedTicket,
            format!("{} has been solved by {}.", channel.name, msg.author.name)
        );
        create_temp_operation(info, msg.channel_id.to_string(), Utc::now() + Duration::hours(1), ActionType::SolvedTicket);
        let final_msg = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Marked as solved!");
                e.description("Ticket will be removed after 1 hour! If you wish to reopen the ticket react with ❌.");
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });

        match final_msg {
            Ok(m) => {
                let _ = m.react(ctx.http.clone(), ReactionType::Unicode(String::from("❌")));
            },
            Err(_) => return Err("Could not create a message.".to_string())
        }

        Ok(())
    }
}
