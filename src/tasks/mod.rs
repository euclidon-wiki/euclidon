use std::sync::Arc;

use tokio::task::JoinHandle;

use crate::App;

pub fn spawn_tasks(_: Arc<App>) -> Vec<JoinHandle<()>> {
    vec![]
}
