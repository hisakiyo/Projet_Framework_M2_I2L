use crate::schema::currencies;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Currency {
    pub id: i32,
    pub symbol: String,
    pub name: String,
}

// Inserable currency
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "currencies"]
pub struct NewCurrency {
    pub symbol: String,
    pub name: String,
}

// currency with price
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct CurrencyWithPrice {
    pub symbol: String,
    pub name: String,
    pub price: BigDecimal,
}

// Updateable currency
#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "currencies"]
pub struct UpdateCurrency {
    pub symbol: String,
    pub name: String,
}

// currency with price and last update
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct CurrencyWithPriceAndLastUpdate {
    pub id: i32,
    pub symbol: String,
    pub name: String,
    pub price: BigDecimal,
    pub last_update: NaiveDateTime,
}