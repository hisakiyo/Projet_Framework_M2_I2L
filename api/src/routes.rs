use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::models::{Currency, User, Price, Transaction};
use crate::schema;
use crate::DbConn;

// get / say hello
#[get("/")]
pub fn index() -> &'static str {
    "Bienvenue sur notre super API"
}

#[get("/users")]
pub fn get_users(conn: DbConn) -> Json<Vec<User>> {
    use schema::users::dsl::*;

    let results = users
        .limit(5)
        .load::<User>(&*conn)
        .expect("Error loading users");

    Json(results)
}