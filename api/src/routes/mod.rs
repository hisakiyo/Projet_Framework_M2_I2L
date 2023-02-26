pub mod user;
pub use user::*;

pub mod currency;
pub use currency::*;

pub mod price;
pub use price::*;

pub mod transaction;
pub use transaction::*;

#[get("/")]
pub fn index() -> &'static str {
    "Bienvenue sur notre super API"
}