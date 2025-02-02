use std::{sync::Arc, time::Duration};

use chrono::Utc;
use tokio::time::sleep;

use crate::{model::user, App};

const SECONDS_IN_MONTH: u64 = 30 * 24 * 3600;

pub async fn cleanup_table_user_sessions(app: Arc<App>) {
    loop {
        if let Err(e) = user::cleanup_sessions(&app.db, Utc::now()) {
            println!("Error occured during cleanup of user_sessions table: {e:?}");
        }

        sleep(Duration::from_secs(SECONDS_IN_MONTH)).await;
    }
}
