use tracing::info;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};

pub struct Logger;

#[rocket::async_trait]
impl Fairing for Logger {
  fn info(&self) -> Info {
    Info {
      name: "logger",
      kind: Kind::Response
    }
  }

  async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
    info!(
      method = req.method().as_str(),
      status = res.status().code,
      path = req.uri().path().as_str(),
    );
  }
}

