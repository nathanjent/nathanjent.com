extern crate dotenv;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    //println!("{:?}", ::std::env::vars().collect::<Vec<_>>());

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect("Error connecting to database")
}

