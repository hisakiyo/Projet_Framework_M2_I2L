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
    pub timestamp: NaiveDateTime,
}