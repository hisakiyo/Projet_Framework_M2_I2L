use crate::schema::prices;
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Price {
    pub id: i32,
    pub currency_id: i32,
    pub price: BigDecimal,
    pub timestamp: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "prices"]
pub struct NewPrice {
    pub currency_id: i32,
    pub price: BigDecimal,
}

// prices with id, currency_id, price, timestamp, symbol, name
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Prices {
    pub id: i32,
    pub currency_id: i32,
    pub price: BigDecimal,
    pub timestamp: NaiveDateTime,
    pub symbol: String,
    pub name: String,
}