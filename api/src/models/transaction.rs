use crate::schema::transactions;
use crate::models::*;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: i32,
    pub user_id: i32,
    pub currency_id: i32,
    pub price: BigDecimal,
    pub quantity: BigDecimal,
    pub transaction_type: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub user_id: i32,
    pub currency_id: i32,
    pub price: BigDecimal,
    pub quantity: BigDecimal,
    pub transaction_type: String,
}

// transactions with currency_id, quantity and type
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct TransactionReceiver {
    pub currency_id: i32,
    pub quantity: BigDecimal,
    pub transaction_type: String,
}

// TransactionWithPriceAndCurrency
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct TransactionWithPriceAndCurrency {
    pub id: i32,
    pub price: BigDecimal,
    pub quantity: BigDecimal,
    pub transaction_type: String,
    pub currency: Currency,
    pub timestamp: NaiveDateTime,
}

// crypto balance
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct CryptoBalance {
    pub currency: Currency,
    pub quantity: BigDecimal,
}