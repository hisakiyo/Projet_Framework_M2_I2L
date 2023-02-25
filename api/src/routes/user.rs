use crate::{
    models::{NewUser, User, LoginUser, UpdatePassword},
    schema,
    DbConn,
};
use rocket::{self, http::{Cookies}};
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue,};
use diesel::prelude::*;
use config::{Config, ConfigError, File};
use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{encode, Header, EncodingKey,DecodingKey,Validation };

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claim {
    pub email: String,
    pub username: String,
    pub exp: usize,
    pub iat: usize,
}

pub(crate) fn get_jwt() -> Result<String, ConfigError> {
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

#[put("/update-password", format = "json", data = "<update_password>")]
pub fn update_password(update_password: Json<UpdatePassword>, cookies: Cookies, conn: DbConn) -> Result<JsonValue, Status> {
    match cookies.get("token") {
        Some(cookie) => {
            let token = cookie.value();
            let decoding_key = DecodingKey::from_secret(get_jwt().unwrap().as_ref());
            let validation = Validation::default();
            let token_data = jsonwebtoken::decode::<Claim>(token, &decoding_key, &validation);
            match token_data {
                Ok(token_data) => {
                    let email = token_data.claims.email.clone();
                    let user = schema::users::table
                        .filter(schema::users::email.eq(&email))
                        .first::<User>(&*conn)
                        .optional()
                        .map_err(|_| Status::InternalServerError)?;

                    let user = match user {
                        Some(user) => user,
                        None => return Err(Status::Unauthorized),
                    };


                    let password_matches = verify(&update_password.old_password, &user.password)
                        .map_err(|_| Status::InternalServerError)?;

                    if !password_matches {
                        return Err(Status::Unauthorized);
                    }

                    let hashed_password = match hash(&update_password.new_password, DEFAULT_COST) {
                        Ok(h) => h,
                        Err(_) => return Err(Status::InternalServerError),
                    };

                    let result = diesel::update(schema::users::table)
                        .filter(schema::users::email.eq(token_data.claims.email))
                        .set(schema::users::password.eq(hashed_password))
                        .execute(&*conn);

                    match result {
                        Ok(_) => Ok(json!({"message": "Password updated successfully"})),
                        Err(_) => Err(Status::InternalServerError),
                    }
                }
                Err(_) => Err(Status::Unauthorized),
            }
        }
        None => Err(Status::Unauthorized),
    }
}
