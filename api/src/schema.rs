// @generated automatically by Diesel CLI.

diesel::table! {
    currencies (id) {
        id -> Integer,
        symbol -> Varchar,
        name -> Varchar,
    }
}

diesel::table! {
    prices (id) {
        id -> Integer,
        currency_id -> Integer,
        price -> Decimal,
        timestamp -> Datetime,
    }
}

diesel::table! {
    transactions (id) {
        id -> Integer,
        user_id -> Integer,
        symbol -> Varchar,
        price -> Decimal,
        quantity -> Decimal,
        date -> Datetime,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(prices -> currencies (currency_id));
diesel::joinable!(transactions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    currencies,
    prices,
    transactions,
    users,
);
