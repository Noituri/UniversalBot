use crate::utils::db::{ServerInfo, get_special_entity_by_type};
use serenity::prelude::Context;
use crate::database::models::SpecialEntityType;
use serenity::model::id::ChannelId;
use crate::command::EMBED_REGULAR_COLOR;

pub fn send_to_mod_logs(ctx: &Context, info: &ServerInfo, title: &str, content: &str) {
    let channel_id = match get_special_entity_by_type(info, SpecialEntityType::ModLogsChannel) {
        Some(ch) => ch.entity_id,
        None => return
    };

    let _ = ChannelId::from(channel_id.parse::<u64>().unwrap())
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(title);
                e.description(content);
                e.color(EMBED_REGULAR_COLOR);
                e
            });
            m
        });
}