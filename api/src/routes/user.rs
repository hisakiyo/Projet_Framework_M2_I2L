use crate::models::{NewUser, User, LoginUser};
use rocket::{http::{Cookie, Cookies}, http::Status};
use rocket_contrib::json::{Json, JsonValue};
use diesel::{self, prelude::*};
extern crate chrono;
use config::{Config, ConfigError, File};


use crate::schema;
use crate::DbConn;
use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{encode, Header, EncodingKey};


// struct claim
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
    use schema::users::dsl::*;

    let results = users
        .limit(5)
        .load::<User>(&*conn)
        .expect("Error loading users");

    Json(results)
}

// Register a new user
// Register a new user
#[post("/register", format = "json", data = "<new_user>")]
pub fn register(new_user: Json<NewUser>, conn: DbConn) -> Result<JsonValue, Status> {
    use schema::users;

    // Vérifier si l'email existe déjà
    let email_exists = users::table
        .filter(users::email.eq(&new_user.email))
        .select(users::id)
        .first::<i32>(&*conn)
        .optional()
        .expect("Error loading user");

    if email_exists.is_some() {
        return Err(Status::Conflict);
    }

    // Hasher le mot de passe
    let hashed_password = match hash(&new_user.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return Err(Status::InternalServerError),
    };

    // Insérer le nouvel utilisateur dans la base de données
    let new_user = NewUser {
        username: new_user.username.clone(),
        email: new_user.email.clone(),
        password: hashed_password,
    };

    let result = diesel::insert_into(users::table)
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

    // Vérifier si l'email existe
    let user = users
        .filter(email.eq(&login_user.email))
        .first::<User>(&*conn)
        .optional()
        .expect("Error loading user");

    if user.is_none() {
        return Err(Status::Unauthorized);
    }

    let user = user.unwrap();

    // Vérifier le mot de passe
    let password_matches = match verify(&login_user.password, &user.password) {
        Ok(m) => m,
        Err(_) => return Err(Status::Unauthorized),
    };

    if !password_matches {
        return Err(Status::Unauthorized);
    }

    // Créer le token JWT
    let expiration = chrono::Utc::now() + chrono::Duration::hours(24);
    let claims = Claim {
        email: user.email.clone(),
        username: user.username.clone(),
        exp: expiration.timestamp() as usize,
        iat: chrono::Utc::now().timestamp() as usize,
    };

    let header = Header::default();
    let encoding_key = EncodingKey::from_secret(get_jwt().unwrap().as_ref());
    let token = match encode(&header, &claims, &encoding_key) {
        Ok(t) => t,
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(json!({
        "message": "User logged in successfully",
        "token": token
    }))
}