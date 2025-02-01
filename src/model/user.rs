use chrono::{DateTime, Utc};
use diesel::{
    query_dsl::methods::FilterDsl, BoolExpressionMethods, ExpressionMethods, RunQueryDsl,
};

use crate::{db::Db, schema::user_sessions, Error};

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
