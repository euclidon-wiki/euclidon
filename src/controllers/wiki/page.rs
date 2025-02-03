use axum::{
    debug_handler,
    extract::{OriginalUri, Path, Query},
    response::{Html, IntoResponse, Redirect, Response},
    Form,
};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use diesel::{connection::LoadConnection, pg::Pg, Connection};
use serde::Deserialize;
use serde_json::json;
use tera::Context;

use crate::{
    model::{
        page::{Content, NewContent, NewPage, NewRevision, Page, Revision},
        user::Session,
    },
    output::Body,
    App, AppState, Error,
};

#[debug_handler(state = AppState)]
pub async fn get(
    AppState(app): AppState,
    uri: OriginalUri,
    mut jar: CookieJar,
    Path(path): Path<String>,
    Query(action): Query<Action>,
) -> Result<(CookieJar, Response), Error> {
    let display_title = path
        .replace('_', " ")
        .trim()
        .split('/')
        .map(str::trim)
        .collect::<String>();
    let query_title = display_title.replace(char::is_whitespace, "_");
    if path != query_title {
        Ok((
            jar,
            Redirect::permanent(&format!("/w/page/{query_title}{}", action.into_query()))
                .into_response(),
        ))
    } else {
        let response = view_page(&app, &mut jar, display_title, query_title, uri, action)?;

        Ok((jar, response))
    }
}

#[debug_handler(state = AppState)]
pub async fn post(
    AppState(app): AppState,
    uri: OriginalUri,
    mut jar: CookieJar,
    Path(path): Path<String>,
    Query(action): Query<Action>,
    Form(edit): Form<EditPage>,
) -> Result<(CookieJar, Response), Error> {
    let display_title = path
        .replace('_', " ")
        .trim()
        .split('/')
        .map(str::trim)
        .collect::<String>();
    let query_title = display_title.replace(char::is_whitespace, "_");

    let conn = &mut app.db.pool.get()?;
    let Some(session) = validate_login(&mut jar, conn)? else {
        // TODO: redirecting to login like this will cause the form submission to be dropped,
        // and so user contribution will probably be lost. Find some way to get around this.
        return Ok((
            jar,
            Redirect::to(&format!(
                "/w/login?redirect_after={}&action={}",
                uri.path(),
                action.into_query(),
            ))
            .into_response(),
        ));
    };

    let content = NewContent::new(Body::Text(edit.content)).insert(conn)?;
    if let Some(mut page) = Page::by_title(&query_title, conn)? {
        let revision =
            NewRevision::new(Some(page.rev_id), content.id, session.user_id, None).insert(conn)?;
        page.set_revision(&revision, conn)?;
    } else {
        let revision = NewRevision::new(None, content.id, session.user_id, None).insert(conn)?;
        NewPage::new(&query_title, revision.id, Some(revision.created_on)).insert(conn)?;
    };

    Ok((
        jar,
        Redirect::to(
            &uri.path_and_query()
                .map(|p| p.as_str().to_string())
                .unwrap_or_else(|| format!("/w/page/{query_title}")),
        )
        .into_response(),
    ))
}

#[derive(Debug, Deserialize)]
pub struct Action {
    #[serde(default, rename = "action")]
    pub kind: Option<ActionKind>,
}

impl Action {
    pub fn into_query(self) -> String {
        match self.kind {
            Some(action) => format!("?action={action}"),
            None => String::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActionKind {
    #[default]
    View,
    Edit,
    Submit,
}

impl ActionKind {
    pub fn as_text(&self) -> &'static str {
        match self {
            Self::View => "view",
            Self::Submit => "submit",
            Self::Edit => "edit",
        }
    }
}

#[derive(Deserialize)]
pub struct EditPage {
    pub content: String,
}

impl std::fmt::Display for ActionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_text())
    }
}

fn view_page(
    app: &App,
    jar: &mut CookieJar,
    display_title: String,
    query_title: String,
    uri: OriginalUri,
    action: Action,
) -> Result<Response, Error> {
    let conn = &mut app.db.pool.get()?;
    let content = get_page_content(&query_title, conn)?.map(Body::into_text);

    if let Some(ActionKind::Edit) = action.kind {
        view_page_editor(app, jar, display_title, query_title, uri, content)
    } else {
        match content {
            Some(content) => view_page_display(app, display_title, query_title, content),
            None => page_not_found(app, display_title),
        }
    }
}

fn get_page_content<C>(title: &str, conn: &mut C) -> Result<Option<Body>, Error>
where
    C: Connection<Backend = Pg> + LoadConnection,
{
    Ok(if let Some(page) = Page::by_title(title, conn)? {
        if let Some(revision) = Revision::by_id(page.rev_id, conn)? {
            if let Some(content) = Content::by_id(revision.content_id, conn)? {
                Some(content.body)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    })
}

fn view_page_display(
    app: &App,
    display_title: String,
    query_title: String,
    content: String,
) -> Result<Response, Error> {
    render_page(
        &app,
        "page/view",
        json!({
            "title": {
                "display": display_title,
                "query": query_title,
            },
            "content": content
        }),
    )
}

fn page_not_found(app: &App, display_title: String) -> Result<Response, Error> {
    render_page(
        app,
        "page/not-found",
        json!({
            "title": {
                "display": display_title
            },
        }),
    )
}

fn view_page_editor(
    app: &App,
    jar: &mut CookieJar,
    display_title: String,
    query_title: String,
    uri: OriginalUri,
    content: Option<String>,
) -> Result<Response, Error> {
    let conn = &mut app.db.pool.get()?;
    Ok(if validate_login(jar, conn)?.is_none() {
        Redirect::to(&format!(
            "/w/login?redirect_after={}&action=edit",
            uri.path(),
        ))
        .into_response()
    } else {
        render_page(
            &app,
            "page/edit",
            json!({
                "title": {
                    "display": display_title,
                    "query": query_title,
                },
                "content": content.unwrap_or_default()
            }),
        )?
    })
}

fn validate_login<C>(jar: &mut CookieJar, conn: &mut C) -> Result<Option<Session>, Error>
where
    C: Connection<Backend = Pg> + LoadConnection,
{
    Ok(if let Some(cookie) = jar.get("euc-user-token") {
        if let Some(session) = Session::from_token(cookie.value(), conn)? {
            if session.is_expired(Utc::now()) {
                *jar = jar.clone().remove("euc-user-token");
                None
            } else {
                Some(session)
            }
        } else {
            None
        }
    } else {
        None
    })
}

fn render_page(app: &App, template: &str, page: serde_json::Value) -> Result<Response, Error> {
    Ok(Html::from(Response::builder().body(app.renderer.render(
        template,
        &Context::from_serialize(json!({
            "site": {
                "title": &app.config.title
            },
            "page": page
        }))?,
    )?)?)
    .into_response())
}
