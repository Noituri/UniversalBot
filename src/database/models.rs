use super::schema::servers;

#[derive(Queryable, Clone)]
pub struct Server {
    pub id: i32,
    pub guildid: String,
    pub prefix: String,
    pub enabledmodules: Vec<String>,
    pub disabledcommands: Vec<String>
}

#[derive(Insertable)]
#[table_name="servers"]
pub struct NewServer {
    pub guildid: String,
    pub enabledmodules: Vec<String>,
    pub disabledcommands: Vec<String>
}