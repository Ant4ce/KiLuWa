use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_postgres_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Need DATABASE_URL env varibale set.");

    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("ERROR connecting to the database : {}", database_url))
}

