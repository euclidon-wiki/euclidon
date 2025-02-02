use std::sync::RwLock;

use base64::{prelude::BASE64_URL_SAFE, Engine};
use chrono::{DateTime, Utc};
use diesel::{
    connection::LoadConnection, pg::Pg, prelude::Queryable, BoolExpressionMethods, Connection,
    ExpressionMethods, Insertable, OptionalExtension, QueryDsl, RunQueryDsl, Selectable,
};
use rand_chacha::ChaCha12Rng;
use rand_core::{RngCore, SeedableRng};

use crate::{
    auth::Password,
    db::Db,
    schema::{user_sessions, users},
    Error,
};

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct Signup<'a> {
    pub name: &'a str,
    pub email: &'a str,
    #[diesel(serialize_as = Password)]
    pub password: &'a str,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
}

impl<'a> Signup<'a> {
    pub fn new(
        name: &'a str,
        email: &'a str,
        password: &'a str,
        created_on: Option<DateTime<Utc>>,
    ) -> Self {
        let created_on = created_on.unwrap_or_else(|| Utc::now());
        Self {
            name,
            email,
            password,
            created_on,
            updated_on: created_on,
        }
    }

    pub fn has_conflict<C>(&self, conn: &mut C) -> Result<bool, Error>
    where
        C: Connection<Backend = Pg> + LoadConnection,
    {
        Ok(0 != users::table
            .filter(users::name.eq(self.name).or(users::email.eq(self.email)))
            .count()
            .get_result::<i64>(conn)?)
    }

    pub fn insert<C>(self, conn: &mut C) -> Result<bool, Error>
    where
        C: Connection<Backend = Pg> + LoadConnection,
    {
        Ok(!self.has_conflict(conn)?
            && 0 != self
                .insert_into(users::table)
                .on_conflict_do_nothing()
                .execute(conn)?)
    }
}

#[derive(Selectable, Queryable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: Password,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
}

impl User {
    pub fn from_name<C>(name: &str, conn: &mut C) -> Result<Option<Self>, Error>
    where
        C: Connection<Backend = Pg> + LoadConnection,
    {
        Ok(users::table
            .filter(users::name.eq(name))
            .first(conn)
            .optional()?)
    }

    pub fn from_email<C>(email: &str, conn: &mut C) -> Result<Option<Self>, Error>
    where
        C: Connection<Backend = Pg> + LoadConnection,
    {
        Ok(users::table
            .filter(users::email.eq(email))
            .first(conn)
            .optional()?)
    }

    pub fn mark_invalid<C>(self, conn: &mut C) -> Result<bool, Error>
    where
        C: Connection<Backend = Pg>,
    {
        Ok(
            0 != diesel::update(users::table.filter(users::id.eq(self.id)))
                .set(users::password.eq(Password::Invalid))
                .execute(conn)?,
        )
    }
}

#[derive(Insertable)]
#[diesel(table_name = user_sessions)]
pub struct Session {
    pub token: String,
    pub user_id: i64,
    pub expire_on: Option<DateTime<Utc>>,
}

impl Session {
    pub fn generate<C>(
        user_id: i64,
        expire_on: Option<DateTime<Utc>>,
        conn: &mut C,
    ) -> Result<Self, Error>
    where
        C: Connection<Backend = Pg> + LoadConnection,
    {
        Ok(Self {
            token: Self::new_token(conn)?,
            user_id,
            expire_on,
        })
    }

    pub fn insert<C>(self, conn: &mut C) -> Result<bool, Error>
    where
        C: Connection<Backend = Pg> + LoadConnection,
    {
        Ok(0 != self
            .insert_into(user_sessions::table)
            .on_conflict_do_nothing()
            .execute(conn)?)
    }

    fn new_token<C>(conn: &mut C) -> Result<String, Error>
    where
        C: Connection<Backend = Pg> + LoadConnection,
    {
        thread_local! {
        static TOKEN_RAND: RwLock<ChaCha12Rng> = RwLock::new(ChaCha12Rng::from_os_rng());
        }

        Ok(loop {
            let mut buf = [0; 16];
            TOKEN_RAND.with(|rand| rand.write().expect("RWLock poisoned").fill_bytes(&mut buf));
            let token = BASE64_URL_SAFE.encode(buf);

            if Self::exists(&token, conn)? {
                continue;
            } else {
                break token;
            }
        })
    }

    fn exists<C>(token: &str, conn: &mut C) -> Result<bool, Error>
    where
        C: Connection<Backend = Pg> + LoadConnection,
    {
        Ok(0 != user_sessions::table
            .filter(user_sessions::token.eq(token))
            .count()
            .get_result::<i64>(conn)?)
    }
}

pub fn cleanup_sessions(db: &Db, now: DateTime<Utc>) -> Result<bool, Error> {
    let mut conn = db.pool.get()?;
    Ok(0 != diesel::delete(
        user_sessions::table.filter(
            user_sessions::expire_on
                .is_not_null()
                .and(user_sessions::expire_on.lt(now)),
        ),
    )
    .execute(&mut conn)?)
}

#[cfg(test)]
mod tests {
    use crate::{app::Config, db::Db, Error};

    use super::Signup;

    #[test]
    fn insert_user_test() -> Result<(), Error> {
        dotenvy::dotenv()?;
        let name = "ahraman1";
        let email = "ahraman1.programming@gmail.com";
        let password = "123456";
        let user = Signup::new(name, email, password, None);
        let config = Config::load()?;
        let db = Db::new(&config)?;
        let res = user.insert(&mut db.pool.get()?)?;
        println!("{}", if res { "success" } else { "failure" });

        Ok(())
    }
}
