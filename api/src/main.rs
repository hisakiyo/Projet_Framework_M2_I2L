#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;

pub mod cors;
pub mod models;
pub mod routes;
pub mod service;
pub mod schema; 

#[database("projet_db")]
pub struct DbConn(diesel::MysqlConnection);
fn main() {
    let r = rocket::ignite()
        .mount("/", routes![
            routes::get_users,
            routes::register,
            routes::login,
            routes::index,
            routes::get_currencies,
            routes::add_currency,
            routes::get_prices,
            routes::get_currencies_with_price,
            routes::update_password,
            routes::get_balance,
            routes::get_transactions,
            routes::new_transaction,
            routes::get_crypto_balance,
            routes::get_last_transactions
        ])
        .attach(DbConn::fairing())
        .attach(cors::CorsFairing);
    
    let rt = Runtime::new().unwrap();
    rt.spawn(async {
        loop {
            let res = service::fetch_currencies().await;
            println!("res: {:?}", res);
            thread::sleep(Duration::from_secs(3600));
        }
    });

    r.launch();
}