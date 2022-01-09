use crate::schema::user;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "user"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}
