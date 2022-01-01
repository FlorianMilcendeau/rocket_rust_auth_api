#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::models::user::{NewUser, User};
use crate::schema::user;
use crate::schema::user::dsl::*;

pub fn create_user(new_user: NewUser, conn: &MysqlConnection) -> QueryResult<User> {
   diesel::insert_into(user::table)
        .values(&new_user)
        .execute(conn)?;

    user::table.find(id).first(conn)
}

pub fn get_user_by_id(user_id: i32, conn: &MysqlConnection) -> QueryResult<User> {
    user::table.find(user_id).first(conn)
}
