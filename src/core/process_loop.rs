// use super::models;

use super::queue;
use super::dispatcher::dispatch;
use tokio::task;

pub fn start() {
    let messages = queue::all();
    for msg in messages {
        task::spawn(dispatch(msg));
    }
}
