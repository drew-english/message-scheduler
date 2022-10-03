#[get("/test")]
pub fn test() -> &'static str {
    "HELLO"
}
