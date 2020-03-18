use serenity::model::channel::{Message, ChannelType, PermissionOverwrite, PermissionOverwriteType, ReactionType};
use serenity::model::Permissions;
use serenity::prelude::Context;
use crate::database::models::SpecialEntityType;
use crate::utils::db::{ServerInfo, ActionType, create_action, get_special_entity_by_type};
use crate::command::{Command, CommandArg, CommandConfig, EMBED_REGULAR_COLOR};
use rand::prelude::*;
use chrono::Utc;

pub struct NewTicketCommand;

impl Command for NewTicketCommand {
    fn name(&self) -> String {
        String::from("new")
    }

    fn desc(&self) -> String {
        String::from("Creates new ticket")
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

    // TODO: Spam protection. Block creating multiple tickets.
    fn exe(&self, ctx: &Context, msg: &Message, info: &ServerInfo) -> Result<(), String> {
        let prefix = &info.server.as_ref().unwrap().prefix;
        let ticket_category = match get_special_entity_by_type(info, SpecialEntityType::TicketsCategory) {
            Some(id) => id.entity_id,
            None => return Err(format!("Tickets' category does not exist. Please use `{}setup tickets`!", prefix))
        };

        let mut rng = rand::thread_rng();
        let ticket_id = rng.gen_range(1000, 10000);
        let result = msg.guild_id.unwrap().create_channel(&ctx.http, |ch| {
            ch.name(format!("ticket-{}", ticket_id));
            ch.kind(ChannelType::Text);
            ch.category(ticket_category.parse::<u64>().unwrap());
            ch.topic(format!("Ticket number: {}. Issued by: {}, Creation Date: {}", ticket_id, msg.author.name, Utc::now().to_rfc2822()));

            let mut perms = Permissions::SEND_MESSAGES;
            perms.insert(Permissions::READ_MESSAGES);
            perms.insert(Permissions::ADD_REACTIONS);
            ch.permissions(vec![PermissionOverwrite {
                allow: perms,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(msg.author.id)
            }]);
            ch
        });

        match result {
            Ok(c) => {
                create_action(
                    info,
                    msg.author.id.to_string(),
                    Some(c.id.to_string()),
                    ActionType::NewTicket,
                    format!("User {} created a ticket-{}.", msg.author.name, ticket_id)
                );

                let result = c.send_message(ctx.http.clone(), |m| {
                    m.embed(|e| {
                        e.title("Ticket has been created!");
                        e.description(format!("Hi <@{}>! Someone from support team will help you out soon! \
                                               Type `{}solved` or react with ✅ to mark this ticket as solved.", msg.author.id.0, prefix));
                        e.color(EMBED_REGULAR_COLOR);
                        e
                    });
                    m
                });

                match result {
                    Ok(m) => {
                        let _ = m.react(ctx.http.clone(), ReactionType::from("✅"));
                    },
                    Err(_) => {}
                }

                let _ = msg.channel_id.send_message(ctx.http.clone(), |m| {
                    m.embed(|e| {
                        e.title("Created a new ticket!");
                        e.description(format!("Your ticket: <#{}>", c.id.0));
                        e.color(EMBED_REGULAR_COLOR);
                        e
                    });
                    m
                });
            }
            Err(_) => return Err(format!("Could not create a new ticket. Check if tickets category is properly setup or use `{}setup tickets`!", prefix))
        }

        Ok(())
    }
}
