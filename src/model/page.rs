use chrono::{DateTime, Utc};
use diesel::{
    connection::LoadConnection, pg::Pg, prelude::Insertable, Connection, ExpressionMethods,
    QueryDsl, Queryable, RunQueryDsl, Selectable,
};

use crate::{
    output::Body,
    schema::{contents, pages, revisions},
    Error,
};

#[derive(Queryable, Selectable)]
#[diesel(check_for_backend(Pg))]
pub struct Page {
    pub id: i64,
    pub title: String,

    pub rev_id: i64,
    pub root_id: i64,

    pub created_on: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = pages, check_for_backend(Pg))]
pub struct NewPage<'a> {
    pub title: &'a str,
    pub rev_id: i64,
    pub root_id: i64,
}

impl<'a> NewPage<'a> {
    pub fn new(title: &'a str, rev_id: i64) -> Self {
        Self {
            title,
            rev_id,
            root_id: rev_id,
        }
    }

    pub fn has_conflict<C>(&self, conn: &mut C) -> Result<bool, Error>
    where
        C: Connection<Backend = Pg> + LoadConnection,
    {
        Ok(0 != pages::table
            .filter(pages::title.eq(self.title))
            .count()
            .get_result::<i64>(conn)?)
    }

    pub fn insert<C>(self, conn: &mut C) -> Result<Page, Error>
    where
        C: Connection<Backend = Pg> + LoadConnection,
    {
        Ok(self
            .insert_into(pages::table)
            .on_conflict_do_nothing()
            .returning(pages::all_columns)
            .get_result(conn)?)
    }
}

#[derive(Queryable, Selectable)]
#[diesel(check_for_backend(Pg))]
pub struct Revision {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub content_id: i64,

    pub user_id: i64,
    pub created_on: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = revisions, check_for_backend(Pg))]
pub struct NewRevision {
    pub parent_id: Option<i64>,
    pub content_id: i64,

    pub user_id: i64,
    pub created_on: DateTime<Utc>,
}

impl NewRevision {
    pub fn new(
        parent_id: Option<i64>,
        content_id: i64,
        user_id: i64,
        created_on: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            parent_id,
            content_id,

            user_id,
            created_on: created_on.unwrap_or_else(|| Utc::now()),
        }
    }

    pub fn insert<C>(self, conn: &mut C) -> Result<Revision, Error>
    where
        C: Connection<Backend = Pg> + LoadConnection,
    {
        Ok(self
            .insert_into(revisions::table)
            .on_conflict_do_nothing()
            .returning(revisions::all_columns)
            .get_result(conn)?)
    }
}

#[derive(Queryable, Selectable)]
#[diesel(check_for_backend(Pg))]
pub struct Content {
    pub id: i64,
    pub body: Body,
}

#[derive(Insertable)]
#[diesel(table_name = contents, check_for_backend(Pg))]
pub struct NewContent {
    pub body: Body,
}

impl NewContent {
    pub fn insert<C>(self, conn: &mut C) -> Result<Content, Error>
    where
        C: Connection<Backend = Pg> + LoadConnection,
    {
        Ok(self
            .insert_into(contents::table)
            .on_conflict_do_nothing()
            .returning(contents::all_columns)
            .get_result(conn)?)
    }
}
