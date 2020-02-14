use super::schema::{roles, servers};

#[derive(Identifiable, Queryable, Clone)]
#[table_name = "servers"]
pub struct Server {
    pub id: i32,
    pub guildid: String,
    pub prefix: String,
    pub enabledmodules: Vec<String>,
    pub disabledcommands: Vec<String>,
}

#[derive(Insertable)]
#[table_name = "servers"]
pub struct NewServer {
    pub guildid: String,
    pub enabledmodules: Vec<String>,
    pub disabledcommands: Vec<String>,
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