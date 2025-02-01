mod db;

use std::sync::Arc;

use tokio::task::JoinHandle;

use crate::App;

pub fn spawn_tasks(app: Arc<App>) -> Vec<JoinHandle<()>> {
    vec![tokio::spawn(db::cleanup_table_user_sessions(app.clone()))]
}
