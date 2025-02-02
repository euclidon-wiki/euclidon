use diesel::{
    r2d2::{ConnectionManager, Pool},
    MultiConnection, PgConnection, QueryResult,
};

use crate::{app::Config, Error};

#[derive(MultiConnection)]
pub enum AnyConn {
    Pg(PgConnection),
}

pub type ConnMan = ConnectionManager<PgConnection>;

pub struct Db {
    pub pool: Pool<ConnMan>,
}

impl Db {
    pub fn new(config: &Config) -> Result<Self, Error> {
        let database_url = &config.database_url;
        let manager = ConnMan::new(database_url);
        let pool = Pool::builder().build(manager)?;

        println!("> database connection established to: {database_url}");
        println!("> active connections: {}", pool.state().connections);
        Ok(Self { pool })
    }
}
