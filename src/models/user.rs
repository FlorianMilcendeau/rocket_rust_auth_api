#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "user"]
pub struct User {
    id: i32,
    email: String,
    password: String,
}

#[derive(Insertable,  Serialize, Deserialize)]
#[table_name = "user"]
pub struct NewUser {
    email: String,
    password: String,
}
