use crate::schema::transactions;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: i32,
    pub user_id: i32,
    pub symbol: String,
    pub price: f64,
    pub quantity: f64,
    pub date: NaiveDateTime,
}