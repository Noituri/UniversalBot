use super::schema::{roles, servers, commands, actions, temp_bans_mutes};
use chrono::{DateTime, Utc, NaiveTime, NaiveDate};

#[derive(Identifiable, Queryable, Clone)]
#[table_name = "servers"]
pub struct Server {
    pub id: i32,
    pub guildid: String,
    pub prefix: String,
    pub enabledmodules: Vec<String>,
}

#[derive(Insertable)]
#[table_name = "servers"]
pub struct NewServer {
    pub guildid: String,
    pub enabledmodules: Vec<String>,
}

#[derive(Identifiable, Queryable, Associations, Clone)]
#[belongs_to(Server, foreign_key = "server_id")]
#[table_name = "roles"]
pub struct Role {
    pub id: i32,
    pub server_id: i32,
    pub role_id: String,
    pub perms: Vec<String>,
}

#[derive(Insertable, Associations)]
#[belongs_to(Server, foreign_key = "server_id")]
#[table_name = "roles"]
pub struct NewRole {
    pub server_id: i32,
    pub role_id: String,
    pub perms: Vec<String>
}

#[derive(Identifiable, Queryable, Associations, Clone)]
#[belongs_to(Server, foreign_key = "server_id")]
#[table_name = "commands"]
pub struct DBCommand {
    pub id: i32,
    pub server_id: i32,
    pub command_name: String,
    pub enabled_channels: Vec<String>
}

#[derive(Insertable, Associations)]
#[belongs_to(Server, foreign_key = "server_id")]
#[table_name = "commands"]
pub struct NewDBCommand {
    pub server_id: i32,
    pub command_name: String,
    pub enabled_channels: Vec<String>
}

#[derive(Identifiable, Queryable, Associations, Clone)]
#[belongs_to(Server, foreign_key = "server_id")]
#[table_name = "actions"]
pub struct Action {
    pub id: i32,
    pub server_id: i32,
    pub action_type: i32,
    pub issuer: String,
    pub target: Option<String>,
    pub message: String,
}

#[derive(Insertable, Associations)]
#[belongs_to(Server, foreign_key = "server_id")]
#[table_name = "actions"]
pub struct NewAction {
    pub server_id: i32,
    pub action_type: i32,
    pub issuer: String,
    pub target: Option<String>,
    pub message: String
}

#[derive(Identifiable, Queryable, Associations, Clone)]
#[belongs_to(Server, foreign_key = "server_id")]
#[table_name = "temp_bans_mutes"]
pub struct TempBanMute {
    pub id: i32,
    pub server_id: i32,
    pub action_type: i32,
    pub user_id: String,
    pub end_date: NaiveDate
}

#[derive(Insertable, Associations)]
#[belongs_to(Server, foreign_key = "server_id")]
#[table_name = "temp_bans_mutes"]
pub struct NewTempBanMute {
    pub server_id: i32,
    pub action_type: i32,
    pub user_id: String,
    pub end_date: NaiveDate
}