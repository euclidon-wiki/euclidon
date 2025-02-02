use chrono::{DateTime, Utc};
use diesel::{
    connection::LoadConnection, pg::Pg, BoolExpressionMethods, Connection, ExpressionMethods,
    Insertable, QueryDsl, RunQueryDsl,
};

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
        let res = users::table
            .filter(users::name.eq(self.name).or(users::email.eq(self.email)))
            .count()
            .get_result::<i64>(conn)?;

        Ok(res != 0)
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

pub fn cleanup_sessions(db: &Db, now: DateTime<Utc>) -> Result<(), Error> {
    let mut conn = db.pool.get()?;
    diesel::delete(
        user_sessions::table.filter(
            user_sessions::expire_on
                .is_not_null()
                .and(user_sessions::expire_on.lt(now)),
        ),
    )
    .execute(&mut conn)?;

    Ok(())
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
