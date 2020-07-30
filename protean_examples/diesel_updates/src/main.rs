//! An example of getting the changes made by a diesel update query
//!
//! This is being built for a specific use case in The Process Foundry, so I'm using it as a stalking
//! horse. I'll clean up the

use anyhow::{Context, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

fn bootstrap_diesel() -> Result<PgConnection> {
    let database_url =
        env::var("DATABASE_URL").context("Postgres does not have a DATABASE_URL must be set")?;
    Ok(PgConnection::establish(&database_url)
        .context(format!("Error connecting to {}", database_url))?)
}

fn load_cache(_json_file: String) -> Result<()> {
    unimplemented!("'load_cache' still needs to be implemented")
}

fn main() {
    dotenv().ok();
    env_logger::init();

    log::info!("Starting up Diesel");
    let _pg_conn = bootstrap_diesel().unwrap();
    log::info!(
        "Loading the data cache:\n{:#?}",
        load_cache("Some_JSON".to_string()).unwrap()
    )
}
