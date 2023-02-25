use crate::{
    models::{ User, Transaction, NewTransaction, Price, TransactionReceiver, TransactionWithPriceAndCurrency, Currency, CryptoBalance },
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
                        if transaction.transaction_type == "buy" {
                            balance -= transaction.quantity * transaction.price;
                        } else {
                            balance += transaction.quantity * transaction.price;
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

// get user crypto balance: how much of each crypto he has (ex: BTC: 0.5, ETH: 1.2)
// It should add all transactions of the user and return the balance. If transaction is type sell, it should subtract the quantity from the balance of the currency
// If transaction is type buy, it should add the quantity to the balance of the currency
#[get("/crypto_balance")]
pub fn get_crypto_balance(cookies: Cookies, conn: DbConn) -> Result<JsonValue, Status> {
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

                    let mut crypto_balance: Vec<CryptoBalance> = Vec::new();
                    for transaction in transactions {
                        let currency = schema::currencies::table
                            .filter(schema::currencies::id.eq(transaction.currency_id))
                            .first::<Currency>(&*conn)
                            .map_err(|_| Status::InternalServerError)?;

                        let mut found = false;
                        for balance in &mut crypto_balance {
                            if balance.currency.id == currency.id {
                                if transaction.transaction_type == "buy" {
                                    balance.quantity += transaction.quantity.clone();
                                } else {
                                    balance.quantity -= transaction.quantity.clone();
                                }
                                found = true;
                            }
                        }
                        if !found {
                            crypto_balance.push(CryptoBalance {
                                currency,
                                quantity: transaction.quantity,
                            });
                        }
                    }
                    
                    crypto_balance.retain(|x| x.quantity != BigDecimal::from(0));

                    Ok(json!({
                        "crypto_balance": crypto_balance,
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

                    let mut transactions_with_price_and_currency: Vec<TransactionWithPriceAndCurrency> = Vec::new();
                    for transaction in transactions {
                        let price = schema::prices::table
                            .filter(schema::prices::currency_id.eq(transaction.currency_id))
                            .order(schema::prices::timestamp.desc())
                            .first::<Price>(&*conn)
                            .optional()
                            .map_err(|_| Status::InternalServerError)?;

                        let currency = schema::currencies::table
                            .filter(schema::currencies::id.eq(transaction.currency_id))
                            .first::<Currency>(&*conn)
                            .optional()
                            .map_err(|_| Status::InternalServerError)?;

                        let transaction_with_price_and_currency = TransactionWithPriceAndCurrency {
                            id: transaction.id,
                            price: price.unwrap().price,
                            quantity: transaction.quantity,
                            transaction_type: transaction.transaction_type,
                            currency: currency.unwrap(),
                            timestamp: transaction.timestamp.clone(),
                        };

                        transactions_with_price_and_currency.push(transaction_with_price_and_currency);
                    }

                    // Sort transactions by timestamp (oldest first)
                    transactions_with_price_and_currency.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

                    Ok(json!({
                        "transactions": transactions_with_price_and_currency,
                    }))
                }
                Err(_) => Err(Status::Unauthorized),
            }
        }
        None => Err(Status::Unauthorized),
    }
}

// get last 5 transactions of the user
#[get("/last_transactions")]
pub fn get_last_transactions(cookies: Cookies, conn: DbConn) -> Result<JsonValue, Status> {
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
                        .order(schema::transactions::timestamp.desc())
                        .limit(5)
                        .load::<Transaction>(&*conn)
                        .map_err(|_| Status::InternalServerError)?;

                    let mut transactions_with_price_and_currency: Vec<TransactionWithPriceAndCurrency> = Vec::new();
                    for transaction in transactions {
                        let price = schema::prices::table
                            .filter(schema::prices::currency_id.eq(transaction.currency_id))
                            .order(schema::prices::timestamp.desc())
                            .first::<Price>(&*conn)
                            .optional()
                            .map_err(|_| Status::InternalServerError)?;

                        let currency = schema::currencies::table
                            .filter(schema::currencies::id.eq(transaction.currency_id))
                            .first::<Currency>(&*conn)
                            .optional()
                            .map_err(|_| Status::InternalServerError)?;

                        let transaction_with_price_and_currency = TransactionWithPriceAndCurrency {
                            id: transaction.id,
                            price: price.unwrap().price,
                            quantity: transaction.quantity,
                            transaction_type: transaction.transaction_type,
                            currency: currency.unwrap(),
                            timestamp: transaction.timestamp.clone(),
                        };

                        transactions_with_price_and_currency.push(transaction_with_price_and_currency);
                    }

                    transactions_with_price_and_currency.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

                    Ok(json!({
                        "transactions": transactions_with_price_and_currency,
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
pub fn new_transaction(cookies: Cookies, conn: DbConn, transaction: Json<TransactionReceiver>) -> Result<JsonValue, Status> {
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

                    // Fetch last price for the currency
                    let last_price = schema::prices::table
                        .filter(schema::prices::currency_id.eq(transaction.currency_id))
                        .order(schema::prices::timestamp.desc())
                        .first::<Price>(&*conn)
                        .optional()
                        .map_err(|_| Status::InternalServerError)?;

                    // if user has enough balance
                    if transaction.transaction_type == "sell" {
                        let transactions = schema::transactions::table
                            .filter(schema::transactions::user_id.eq(user.id))
                            .load::<Transaction>(&*conn)
                            .map_err(|_| Status::InternalServerError)?;

                        let mut crypto_balance: Vec<CryptoBalance> = Vec::new();
                        for transaction in transactions {
                            let currency = schema::currencies::table
                                .filter(schema::currencies::id.eq(transaction.currency_id))
                                .first::<Currency>(&*conn)
                                .map_err(|_| Status::InternalServerError)?;

                            let mut found = false;
                            for balance in &mut crypto_balance {
                                if balance.currency.id == currency.id {
                                    if transaction.transaction_type == "buy" {
                                        balance.quantity += transaction.quantity.clone();
                                    } else {
                                        balance.quantity -= transaction.quantity.clone();
                                    }
                                    found = true;
                                }
                            }
                            if !found {
                                crypto_balance.push(CryptoBalance {
                                    currency,
                                    quantity: transaction.quantity,
                                });
                            }
                        }

                        let mut found = false;
                        for balance in crypto_balance {
                            if balance.currency.id == transaction.currency_id {
                                if balance.quantity < transaction.quantity {
                                    return Err(Status::BadRequest);
                                }
                                found = true;
                            }
                        }
                        if !found {
                            return Err(Status::BadRequest);
                        }
                    // else, if buy, user must have enough money (only in USD. with prices). take the same code as #[get("/balance")] do price*quantity
                    } else {
                        let transactions = schema::transactions::table
                            .filter(schema::transactions::user_id.eq(user.id))
                            .load::<Transaction>(&*conn)
                            .map_err(|_| Status::InternalServerError)?;

                        let mut total_balance: BigDecimal = 0.0.into();
                        for transaction in transactions {
                            let price = schema::prices::table
                                .filter(schema::prices::currency_id.eq(transaction.currency_id))
                                .order(schema::prices::timestamp.desc())
                                .first::<Price>(&*conn)
                                .optional()
                                .map_err(|_| Status::InternalServerError)?;

                            if transaction.transaction_type == "buy" {
                                total_balance -= price.unwrap().price * transaction.quantity;
                            } else {
                                total_balance += price.unwrap().price * transaction.quantity;
                            }
                        }

                        if total_balance + (last_price.clone().unwrap().price * transaction.quantity.clone()) >= 10000.0.into() {
                            return Err(Status::BadRequest);
                        }
                    }   


                    let new_transaction = NewTransaction {
                        user_id: user.id,
                        currency_id: transaction.currency_id,
                        price: last_price.unwrap().price,
                        quantity: transaction.quantity.clone(),
                        transaction_type: transaction.transaction_type.clone(),
                    };

                    let transaction = diesel::insert_into(schema::transactions::table)
                        .values(&new_transaction)
                        .execute(&*conn)
                        .map_err(|_| Status::InternalServerError)?;

                    Ok(json!({
                        "status": "success",
                        "transaction": transaction,
                    }))
                }
                Err(_) => Err(Status::Unauthorized),
            }
        }
        None => Err(Status::Unauthorized),
    }
}