//! An example of getting the changes made by a diesel update query
//!
//! This is being built for a specific use case in The Process Foundry, so I'm using it as a stalking
//! horse. I'll clean up the

use anyhow::{Context, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod cache;
pub mod error;
pub mod postgres;
pub mod replicant;

use cache::*;
use error::FHLError;
use postgres::*;
use protean::Patchwork;
use replicant::*;

fn _bootstrap_diesel() -> Result<PgConnection> {
    let database_url =
        env::var("DATABASE_URL").context("Postgres does not have a DATABASE_URL must be set")?;
    Ok(PgConnection::establish(&database_url)
        .context(format!("Error connecting to {}", database_url))?)
}

fn _load_cache(_json_file: String) -> Result<()> {
    unimplemented!("'load_cache' still needs to be implemented")
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let mut fhl_cache = FHLCache::new();
    log::debug!("fhl_cache:\n{:#?}", fhl_cache);

    // Create new postgres Test
    let test1 = Test {
        guid: "Test1".to_string(),
        value: "Test Value 1".to_string(),
    };

    fhl_cache.upsert("test", test1.to_patch().unwrap()).unwrap();
    // Apply to cache
    // Get postgres Test from cache

    // Register Postgres as a subscriber to the cache for all

    // Push postgres Test to postgres
    // Direct query to postgres should have both

    // Add GoogleDocs as a subscriber to the cache
    // See it pull all postgres Tests from the cache
    // Add a postgres Test to GoogleDocs, watch it move to FHLCache and Postgres
    // Pull all postgres Tests from GoogleDocs and watch them all changes load into postgres

    // Remove Tests from postgres and implement the rest of the mapping syncing
    // Add GraphQL to postgres
}
