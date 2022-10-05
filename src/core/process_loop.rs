use crate::core::dispatcher::dispatch;
use crate::core::queue;
use tokio::task;

pub fn start() {
    let messages = queue::front(Option::<usize>::Some(1));
    for msg in messages {
        task::spawn(dispatch(msg));
    }
}
