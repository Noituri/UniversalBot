use crate::database::models::{Server, NewServer};
use crate::database::schema::servers;
use diesel::associations::HasTable;
use diesel::RunQueryDsl;
use crate::database::get_db_con;

pub fn create_server(guild_id: String, enabled_modules: Vec<String>, enabled_cmds: Vec<String>) -> Server {
    let new_server = NewServer {
        guildid: guild_id,
        enabledmodules: enabled_modules,
        enabledcommands: enabled_cmds
    };

    diesel::insert_into(servers::table)
        .values(&new_server)
        .get_result(&get_db_con().get().expect("Pool!"))
        .expect("Error occurred while inserting new server")
}