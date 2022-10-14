use std::sync::Arc;

use crate::core::dispatcher::dispatch;
use crate::core::queue;
use chrono::{DateTime, Utc};
use tokio::{
    sync::mpsc,
    task::{self, JoinHandle},
    time::sleep,
};
use tracing::error;

const BATCH_PROCESS_LIMIT: i64 = 50;

struct ProcessData {
    db_pool: sqlx::Pool<sqlx::Postgres>,
    control_tx: mpsc::Sender<i8>,
    msg_delivery_tx: mpsc::UnboundedSender<Option<DateTime<Utc>>>,
}

impl ProcessData {
    pub fn new(
        db_pool: sqlx::Pool<sqlx::Postgres>,
        control_tx: mpsc::Sender<i8>,
        msg_delivery_tx: mpsc::UnboundedSender<Option<DateTime<Utc>>>,
    ) -> Self {
        Self {
            db_pool,
            control_tx,
            msg_delivery_tx,
        }
    }

    async fn control_ch_send(&self, n: i8) {
        self.control_tx.send(n).await.unwrap();
    }

    async fn msg_delivery_ch_send(&self, date_time: Option<DateTime<Utc>>) {
        self.msg_delivery_tx.send(date_time).unwrap();
    }
}

pub fn run(db_pool: sqlx::Pool<sqlx::Postgres>) -> mpsc::UnboundedSender<Option<DateTime<Utc>>> {
    let (control_tx, control_rx) = mpsc::channel(50);
    let (msg_delivery_tx, msg_delivery_rx) = mpsc::unbounded_channel();
    let p_data = Arc::new(ProcessData::new(
        db_pool,
        control_tx,
        msg_delivery_tx.clone(),
    ));

    task::spawn(main_loop(p_data.clone(), control_rx));
    task::spawn(msg_delivery_process(p_data, msg_delivery_rx));

    msg_delivery_tx
}

async fn main_loop(p_data: Arc<ProcessData>, mut control_rx: mpsc::Receiver<i8>) {
    loop {
        find_and_process_messages(p_data.clone()).await;
        control_rx.recv().await.unwrap();
    }
}

async fn find_and_process_messages(p_data: Arc<ProcessData>) {
    let messages = queue::needing_delivery(&p_data.db_pool, Some(BATCH_PROCESS_LIMIT)).await;
    let message_count = messages.len();

    let mut handles = Vec::new();
    for msg in messages {
        handles.push(task::spawn(dispatch(msg, p_data.db_pool.clone())));
    }

    for handle in handles {
        match handle.await {
            Ok(_) => (),
            Err(err) => error!(
                error = err.to_string(),
                "[ProcessLoop] Failed to dispatch message"
            ),
        }
    }

    if (message_count as i64) == BATCH_PROCESS_LIMIT {
        p_data.control_ch_send(0).await;
    } else {
        p_data.msg_delivery_ch_send(None).await;
    }
}

async fn msg_delivery_process(
    p_data: Arc<ProcessData>,
    mut msg_delivery_rx: mpsc::UnboundedReceiver<Option<DateTime<Utc>>>,
) {
    let mut next_message_delivery = Utc::now();
    let mut control_timer_handle: Option<JoinHandle<()>> = None;
    loop {
        let mut refresh_cache = false;
        let new_delivery_time = match msg_delivery_rx.recv().await.unwrap() {
            Some(delivery_time) => delivery_time,
            None => {
                refresh_cache = true;
                fetch_next_message_delivery(p_data.clone()).await
            },
        };

        if refresh_cache || new_delivery_time < next_message_delivery {
            next_message_delivery = new_delivery_time;
            if let Some(handle) = control_timer_handle.as_ref() {
                handle.abort();
            }

            match (new_delivery_time - Utc::now()).to_std() {
                Ok(duration) => {
                    let task_p_data = p_data.clone();
                    control_timer_handle = Some(task::spawn(async move {
                        sleep(duration).await;
                        task_p_data.control_ch_send(0).await;
                    }));
                },
                Err(_) => p_data.control_ch_send(0).await,
            }
        }
    }
}

async fn fetch_next_message_delivery(p_data: Arc<ProcessData>) -> DateTime<Utc> {
    match queue::next_delivery_time(&p_data.db_pool).await {
        Ok(next_delivery_time) => {
            next_delivery_time.unwrap_or_else(|| Utc::now() + chrono::Duration::days(365))
        }
        Err(err) => {
            error!(error = err.to_string(), "[ProcessLoop] Error fetching next scheduled message, scheduling loop 10 seconds from now");
            Utc::now() + chrono::Duration::seconds(10)
        }
    }
}
