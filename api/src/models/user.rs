use crate::schema::users;
use diesel::Identifiable;

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable, AsChangeset)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl Identifiable for User {
    type Id = i32;

    fn id(self) -> Self::Id {
        self.id
    }
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

// LoginUser
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

// UpdatePassword
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePassword {
    pub old_password: String,
    pub new_password: String,
}