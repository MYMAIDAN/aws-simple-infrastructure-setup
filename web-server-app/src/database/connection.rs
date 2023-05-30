use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection(database_url: &str) -> PgConnection{

    PgConnection::establish(database_url).unwrap_or_else(|_| panic!("Error connection to {}", database_url))

}
