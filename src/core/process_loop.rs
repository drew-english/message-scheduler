use crate::core::dispatcher::dispatch;
use crate::core::queue;
use tokio::task;
use tracing::warn;

const BATCH_PROCESS_LIMIT: i64 = 1;

pub async fn start(db_pool: sqlx::Pool<sqlx::Postgres>) {
    loop {
        find_and_process_messages(&db_pool).await;
        tokio::time::sleep(tokio::time::Duration::new(10, 0)).await;
    }
}

async fn find_and_process_messages(db_pool: &sqlx::Pool<sqlx::Postgres>) {
    let messages = queue::needing_delivery(db_pool, Some(BATCH_PROCESS_LIMIT)).await;
    if messages.len() == 0 {
        warn!("[ProcessLoop] No messages found");
        return;
    }

    for msg in messages {
        task::spawn(dispatch(msg, db_pool.clone()));
    }
}
