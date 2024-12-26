use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify};
use diesel::prelude::*;
use serde_json::json;

use crate::db::DbPool;
use crate::models::{NewUser, User};

pub async fn register(pool: web::Data<DbPool>, user: web::Json<NewUser<'_>>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    let hashed_password = hash(user.password, 4).unwrap();
    let new_user = NewUser {
        username: user.username,
        password: &hashed_password,
    };

    diesel::insert_into(crate::schema::users::table)
        .values(&new_user)
        .execute(&conn)
        .expect("Error saving new user");

    HttpResponse::Created().json(json!({"message": "User created"}))
}

pub async fn login(pool: web::Data<DbPool>, user: web::Json<NewUser<'_>>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    let result: Result<User, diesel::result::Error> = users::table
        .filter(users::username.eq(user.username))
        .first(&conn);

    match result {
        Ok(existing_user) => {
            if verify(user.password, &existing_user.password).unwrap() {
                HttpResponse::Ok().json(json!({"message": "Login successful"}))
            } else {
                HttpResponse::Unauthorized().json(json!({"message": "Invalid password"}))
            }
        }
        Err(_) => HttpResponse::NotFound().json(json!({"message": "User not found"})),
    }
}
