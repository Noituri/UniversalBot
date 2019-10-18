#[derive(Queryable)]
pub struct Server {
    pub id: i32,
    pub guildid: String,
    pub ownerid: String,
    pub enabledmodules: Vec<String>,
    pub enabledcommands: Vec<String>
}