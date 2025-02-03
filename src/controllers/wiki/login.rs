use axum::{
    debug_handler,
    response::{Html, IntoResponse, Redirect, Response},
    Form,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use chrono::Utc;
use diesel::{connection::LoadConnection, pg::Pg, Connection};
use serde::Deserialize;
use serde_json::json;
use tera::Context;

use crate::{
    model::user::{Session, User},
    AppState, Error,
};

#[debug_handler(state = AppState)]
pub async fn get(AppState(app): AppState) -> Result<Response, Error> {
    Ok(Html::from(
        Response::builder().body(
            app.renderer
                .render("login", &Context::from_serialize(json!({}))?)?,
        )?,
    )
    .into_response())
}

#[debug_handler(state = AppState)]
pub async fn post(
    AppState(app): AppState,
    mut jar: CookieJar,
    Form(data): Form<LoginData>,
) -> Result<(CookieJar, Response), Error> {
    let status = if jar.get("euc-user-token").is_none() {
        let conn = &mut app.db.pool.get()?;
        if let Some(login) = data.load(conn)? {
            match login.password.compare(&data.password) {
                Ok(res) => {
                    if res {
                        let session = Session::generate(login.id, None, conn)?;
                        jar = jar.add(Cookie::new("euc-user-token", session.token.clone()));

                        _ = session.insert(conn)?;
                        _ = login.mark_updated(Utc::now(), conn)?;
                    }

                    res.into()
                }
                Err(_) => {
                    login.set_invalid(conn)?;
                    LoginStatus::Failure
                }
            }
        } else {
            LoginStatus::Failure
        }
    } else {
        LoginStatus::Duplicate
    };

    Ok((
        jar,
        Redirect::to(&format!("/w/login?{}", status.into_query())).into_response(),
    ))
}

#[derive(Deserialize)]
pub struct LoginData {
    pub id: String,
    pub password: String,
}

impl LoginData {
    fn load<C>(&self, conn: &mut C) -> Result<Option<User>, Error>
    where
        C: Connection<Backend = Pg> + LoadConnection,
    {
        match self.kind() {
            LoginKind::Username => User::by_name(&self.id, conn),
            LoginKind::Email => User::by_email(&self.id, conn),
        }
    }

    fn kind(&self) -> LoginKind {
        if self.id.contains('@') {
            LoginKind::Email
        } else {
            LoginKind::Username
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginKind {
    Username,
    Email,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginStatus {
    Failure,
    Duplicate,
    Success,
}

impl LoginStatus {
    pub fn into_query(self) -> String {
        format!("status={self}")
    }
}

impl std::fmt::Display for LoginStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Failure => write!(f, "failure"),
            Self::Success => write!(f, "success"),
            Self::Duplicate => write!(f, "duplicate"),
        }
    }
}

impl From<bool> for LoginStatus {
    fn from(value: bool) -> Self {
        value.then_some(Self::Success).unwrap_or(Self::Failure)
    }
}
