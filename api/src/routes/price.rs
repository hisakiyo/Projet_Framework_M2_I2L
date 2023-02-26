use crate::{
    models::Price,
    schema,
    DbConn,
};
use rocket_contrib::json::Json;
use diesel::prelude::*;

#[get("/prices")]
pub fn get_prices(conn: DbConn) -> Json<Vec<Price>> {
    let results = schema::prices::table
        .load::<Price>(&*conn)
        .expect("Error loading prices");

    Json(results)
}
