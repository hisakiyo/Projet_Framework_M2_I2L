use chrono::NaiveDateTime;

use crate::schema::users;

use crate::schema::currencies;

use crate::schema::prices;

use crate::schema::transactions;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Currency {
    pub id: i32,
    pub symbol: String,
    pub name: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Price {
    pub id: i32,
    pub currency_id: i32,
    pub price: f64,
    pub timestamp: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: i32,
    pub user_id: i32,
    pub symbol: String,
    pub price: f64,
    pub quantity: f64,
    pub date: NaiveDateTime,
}