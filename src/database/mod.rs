pub mod models;
pub mod schema;

use diesel::{PgConnection};
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{ConnectionManager, Pool};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = connect();
}

fn connect() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);
    Pool::builder().build(manager).expect("Could not create pool for database!")
}

pub fn get_db_con() -> Pool<ConnectionManager<PgConnection>> {
    DB_POOL.clone()
}