use crate::schema::transactions;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: i32,
    pub user_id: i32,
    pub symbol: String,
    pub price: BigDecimal,
    pub quantity: BigDecimal,
    pub transaction_type: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub user_id: i32,
    pub symbol: String,
    pub price: BigDecimal,
    pub quantity: BigDecimal,
    pub transaction_type: String,
    pub timestamp: NaiveDateTime,
}