use super::schema::servers;

#[derive(Queryable)]
pub struct Server {
    pub id: i32,
    pub guildid: String,
    pub enabledmodules: Vec<String>,
    pub enabledcommands: Vec<String>
}

#[derive(Insertable)]
#[table_name="servers"]
pub struct NewServer {
    pub guildid: String,
    pub enabledmodules: Vec<String>,
    pub enabledcommands: Vec<String>
}