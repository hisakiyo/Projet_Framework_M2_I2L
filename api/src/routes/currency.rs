use crate::{
    models::{Currency, NewCurrency, NewPrice, CurrencyWithPrice,CurrencyWithPriceAndLastUpdate},
    schema,
    DbConn,
};
use rocket::{http::Status};
use rocket_contrib::json::{Json};
use diesel::prelude::*;

#[get("/currencies")]
pub fn get_currencies(conn: DbConn) -> Json<Vec<Currency>> {
    let results = schema::currencies::table
        .load::<Currency>(&*conn)
        .expect("Error loading currencies");

    Json(results)
}

#[post("/currencies", data = "<currency>")]
pub fn add_currency(conn: DbConn, currency: Json<CurrencyWithPrice>) -> Result<Json<Currency>, Status> {
    let existing_currencies = schema::currencies::table
        .filter(schema::currencies::symbol.eq(&currency.symbol))
        .load::<Currency>(&*conn)
        .map_err(|_| Status::InternalServerError)?;

    if let Some(existing_currency) = existing_currencies.first() {
        let new_price = NewPrice {
            currency_id: existing_currency.id,
            price: currency.price.clone(),
        };
        diesel::insert_into(schema::prices::table)
            .values(&new_price)
            .execute(&*conn)
            .map_err(|_| Status::InternalServerError)?;

        return Err(Status::Conflict);
    } else {
        let new_currency = NewCurrency {
            symbol: currency.symbol.clone(),
            name: currency.name.clone(),
        };
    
        diesel::insert_into(schema::currencies::table)
            .values(&new_currency)
            .execute(&*conn)
            .map_err(|_| Status::InternalServerError)?;

        let new_currency = schema::currencies::table
            .filter(schema::currencies::symbol.eq(&currency.symbol))
            .first::<Currency>(&*conn)
            .map_err(|_| Status::InternalServerError)?;

        let new_price = NewPrice {
            currency_id: new_currency.id,
            price: currency.price.clone(),
        };
        println!("new_price: {:?}", new_price);
        
        diesel::insert_into(schema::prices::table)
            .values(&new_price)
            .execute(&*conn)
            .map_err(|_| Status::InternalServerError)?;

        let response = Json(new_currency);
        Ok(response)
    }
}

#[get("/currencies_with_price")]
pub fn get_currencies_with_price(conn: DbConn) -> Json<Vec<CurrencyWithPriceAndLastUpdate>> {
    use diesel::dsl::sql;

    let results = schema::currencies::table
        .inner_join(schema::prices::table)
        .select((
            schema::currencies::id,
            schema::currencies::symbol,
            schema::currencies::name,
            schema::prices::price,
            sql::<diesel::sql_types::Timestamp>("MAX(timestamp)"),
        ))
        .group_by(schema::currencies::id)
        .load::<CurrencyWithPriceAndLastUpdate>(&*conn)
        .expect("Error loading currencies");

    Json(results)
}


