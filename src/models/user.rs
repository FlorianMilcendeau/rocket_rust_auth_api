use rocket::serde::{Deserialize, Serialize};
use crate::schema::user;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "user"]
pub struct NewUser {
    pub email: String,
    pub password: String,
}
