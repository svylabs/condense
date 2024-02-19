use crate::signers::Signer;
use std::sync::{Arc, Mutex};
use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl AppState {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
                .max_size(10)
                .build(manager)
                .expect("Failed to create pool.");
        AppState {
            pool,
        }
    }

}