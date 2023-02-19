pub mod user;
pub use user::*;

#[get("/")]
pub fn index() -> &'static str {
    "Bienvenue sur notre super API"
}