use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
#[allow(unused_imports)]
use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
    State,
};
use std::{env, ops::Deref};

pub mod schema;

// An alias to the type for a pool of Diesel Postgres Connection
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn connect() -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(
        env::var("DATABASE_URL").expect("DATABASE_URL envar must be set"),
    );
    Pool::new(manager).expect("Failed to create pool")
}

pub struct Connection(pub PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for Connection {
    type Target = PgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
