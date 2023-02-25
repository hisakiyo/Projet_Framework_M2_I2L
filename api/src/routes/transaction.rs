use crate::{
    models::{ User, Transaction, NewTransaction},
    schema,
    DbConn,
};
use bigdecimal::BigDecimal;
use rocket::{self, http::{Cookie, Cookies}, Data};
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue,};
use diesel::prelude::*;
use config::{Config, ConfigError, File};
use rocket::request::{self, Request, FromRequest,Outcome};
use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{encode, Header, EncodingKey,DecodingKey,Validation };
use chrono::{Utc, Duration, NaiveDateTime};
use crate::routes::user::Claim;
use crate::routes::user::get_jwt;

// get starting balance
fn get_starting_balance() -> Result<BigDecimal, ConfigError> {
    let mut config = Config::default();
    config.merge(File::with_name("config"))?;

    let starting_balance: BigDecimal = config.get("crypto.starting_balance")?;
    Ok(starting_balance)
}


// Get balance of the user. it combines all his transactions and returns the balance
#[get("/balance")]
pub fn get_balance(cookies: Cookies, conn: DbConn) -> Result<JsonValue, Status> {
    match cookies.get("token") {
        Some(cookie) => {
            let token = cookie.value();
            let decoding_key = DecodingKey::from_secret(get_jwt().unwrap().as_ref());
            let validation = Validation::default();
            let token_data = jsonwebtoken::decode::<Claim>(token, &decoding_key, &validation);
            match token_data {
                Ok(token_data) => {
                    let email = token_data.claims.email.clone();
                    println!("Email: {}", email);
                    let user = schema::users::table
                        .filter(schema::users::email.eq(&email))
                        .first::<User>(&*conn)
                        .optional()
                        .map_err(|_| Status::InternalServerError)?;

                    let user = match user {
                        Some(user) => user,
                        None => return Err(Status::Unauthorized),
                    };

                    let transactions = schema::transactions::table
                        .filter(schema::transactions::user_id.eq(user.id))
                        .load::<Transaction>(&*conn)
                        .map_err(|_| Status::InternalServerError)?;

                    // get starting balance from get_starting_balance() function
                    let mut balance = get_starting_balance().unwrap();
                    for transaction in transactions {
                        if transaction.transaction_type == "credit" {
                            balance += transaction.quantity * transaction.price;
                        } else {
                            balance -= transaction.quantity * transaction.price;
                        }
                    }

                    Ok(json!({
                        "balance": balance,
                    }))
                }
                Err(_) => Err(Status::Unauthorized),
            }
        }
        None => Err(Status::Unauthorized),
    }
}

// Get all transactions of the user
#[get("/transactions")]
pub fn get_transactions(cookies: Cookies, conn: DbConn) -> Result<JsonValue, Status> {
    match cookies.get("token") {
        Some(cookie) => {
            let token = cookie.value();
            let decoding_key = DecodingKey::from_secret(get_jwt().unwrap().as_ref());
            let validation = Validation::default();
            let token_data = jsonwebtoken::decode::<Claim>(token, &decoding_key, &validation);
            match token_data {
                Ok(token_data) => {
                    let email = token_data.claims.email.clone();
                    let user = schema::users::table
                        .filter(schema::users::email.eq(&email))
                        .first::<User>(&*conn)
                        .optional()
                        .map_err(|_| Status::InternalServerError)?;

                    let user = match user {
                        Some(user) => user,
                        None => return Err(Status::Unauthorized),
                    };

                    let transactions = schema::transactions::table
                        .filter(schema::transactions::user_id.eq(user.id))
                        .load::<Transaction>(&*conn)
                        .map_err(|_| Status::InternalServerError)?;

                    Ok(json!({
                        "transactions": transactions,
                    }))
                }
                Err(_) => Err(Status::Unauthorized),
            }
        }
        None => Err(Status::Unauthorized),
    }
}

// new transaction
#[post("/transaction", format = "application/json", data = "<transaction>")]
pub fn new_transaction(cookies: Cookies, conn: DbConn, transaction: Json<NewTransaction>) -> Result<JsonValue, Status> {
    match cookies.get("token") {
        Some(cookie) => {
            let token = cookie.value();
            let decoding_key = DecodingKey::from_secret(get_jwt().unwrap().as_ref());
            let validation = Validation::default();
            let token_data = jsonwebtoken::decode::<Claim>(token, &decoding_key, &validation);
            match token_data {
                Ok(token_data) => {
                    let email = token_data.claims.email.clone();
                    let user = schema::users::table
                        .filter(schema::users::email.eq(&email))
                        .first::<User>(&*conn)
                        .optional()
                        .map_err(|_| Status::InternalServerError)?;

                    let user = match user {
                        Some(user) => user,
                        None => return Err(Status::Unauthorized),
                    };

                    let new_transaction = NewTransaction {
                        user_id: user.id,
                        symbol: transaction.symbol.clone(),
                        price: transaction.price.clone(),
                        quantity: transaction.quantity.clone(),
                        transaction_type: transaction.transaction_type.clone(),
                        timestamp: transaction.timestamp.clone(),
                    };

                    diesel::insert_into(schema::transactions::table)
                        .values(&new_transaction)
                        .execute(&*conn)
                        .map_err(|_| Status::InternalServerError)?;

                    Ok(json!({
                        "status": "success",
                    }))
                }
                Err(_) => Err(Status::Unauthorized),
            }
        }
        None => Err(Status::Unauthorized),
    }
}