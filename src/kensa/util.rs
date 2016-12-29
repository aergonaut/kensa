use errors::*;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> Result<PgConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").chain_err(|| "DATABASE_URL must be set")?;
    PgConnection::establish(&database_url).chain_err(|| "Could not connect to database")
}
