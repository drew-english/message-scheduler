pub mod middlware;
mod routes;

pub fn routes() -> Vec<rocket::Route> {
  routes![
    routes::test,
  ]
}
