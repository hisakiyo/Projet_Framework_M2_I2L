use crate::{
    models::{NewUser, User, LoginUser},
    schema,
    DbConn,
};
use rocket::{http::{Cookie, Cookies},http::Status};
use rocket_contrib::json::{Json, JsonValue};
use diesel::prelude::*;
use config::{Config, ConfigError, File};
use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
struct Claim {
    email: String,
    username: String,
    exp: usize,
    iat: usize,
}

fn get_jwt() -> Result<String, ConfigError> {
    let mut config = Config::default();
    config.merge(File::with_name("config"))?;

    let jwt_config: String = config.get("jwt.secret")?;
    Ok(jwt_config)
}

#[get("/users")]
pub fn get_users(conn: DbConn) -> Json<Vec<User>> {
    let results = schema::users::table
        .limit(5)
        .load::<User>(&*conn)
        .expect("Error loading users");

    Json(results)
}

#[post("/register", format = "json", data = "<new_user>")]
pub fn register(new_user: Json<NewUser>, conn: DbConn) -> Result<JsonValue, Status> {
    let email_exists = schema::users::table
        .filter(schema::users::email.eq(&new_user.email))
        .select(schema::users::id)
        .first::<i32>(&*conn)
        .optional()
        .expect("Error loading user");

    if email_exists.is_some() {
        return Err(Status::Conflict);
    }

    let hashed_password = match hash(&new_user.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return Err(Status::InternalServerError),
    };

    let new_user = NewUser {
        username: new_user.username.clone(),
        email: new_user.email.clone(),
        password: hashed_password,
    };

    let result = diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(&*conn);

    match result {
        Ok(_) => Ok(json!({"message": "User registered successfully"})),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/login", format = "json", data = "<login_user>")]
pub fn login(login_user: Json<LoginUser>, conn: DbConn) -> Result<JsonValue, Status> {
    use schema::users::dsl::*;

    let user = users
        .filter(email.eq(&login_user.email))
        .first::<User>(&*conn)
        .optional()
        .map_err(|_| Status::InternalServerError)?;

    let user = match user {
        Some(user) => user,
        None => return Err(Status::Unauthorized),
    };

    let password_matches = verify(&login_user.password, &user.password)
        .map_err(|_| Status::InternalServerError)?;

    if !password_matches {
        return Err(Status::Unauthorized);
    }

    let expiration = chrono::Utc::now() + chrono::Duration::hours(24);
    let claims = Claim {
        email: user.email.clone(),
        username: user.username.clone(),
        exp: expiration.timestamp() as usize,
        iat: chrono::Utc::now().timestamp() as usize,
    };

    let header = Header::default();
    let encoding_key = EncodingKey::from_secret(get_jwt().unwrap().as_ref());
    let token = encode(&header, &claims, &encoding_key)
        .map_err(|_| Status::InternalServerError)?;

    Ok(json!({
        "message": "User logged in successfully",
        "token": token,
    }))
}
