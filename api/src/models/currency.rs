use crate::schema::currencies;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Currency {
    pub id: i32,
    pub symbol: String,
    pub name: String,
}