pub mod models;
pub mod schema;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Unable to connect to {}", database_url))
}
