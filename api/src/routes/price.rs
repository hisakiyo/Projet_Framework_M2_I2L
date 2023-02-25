use crate::{
    models::{Price, NewPrice, CurrencyWithPrice},
    schema,
    DbConn,
};
use rocket::{http::{Cookie, Cookies},http::Status};
use rocket_contrib::json::{Json, JsonValue};
use diesel::prelude::*;
use chrono::{Utc, Duration};

// get all prices, also add currency name and symbol from price.currency_id
#[get("/prices")]
pub fn get_prices(conn: DbConn) -> Json<Vec<Price>> {
    let results = schema::prices::table
        .load::<Price>(&*conn)
        .expect("Error loading prices");

    Json(results)
}
