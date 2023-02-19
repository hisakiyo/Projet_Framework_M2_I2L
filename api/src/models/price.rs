use crate::schema::prices;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Price {
    pub id: i32,
    pub currency_id: i32,
    pub price: f64,
    pub timestamp: NaiveDateTime,
}