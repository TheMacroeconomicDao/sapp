extern crate actix_web;
extern crate serde;
extern crate rusqlite;
extern crate actix_identity;
extern crate ipfs_api;
extern crate bcrypt;

use actix_web::{web, App, HttpServer, HttpResponse, Error};
use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};
use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection};
use ipfs_api::IpfsClient;
use bcrypt::{hash, DEFAULT_COST, verify};

#[derive(Deserialize)]
struct UserInput {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct Message {
    from: String,
    to: String,
    content: String,
}

async fn login(
    id: Identity,
    data: web::Json<UserInput>,
    conn: web::Data<Connection>,
) -> Result<HttpResponse, Error> {
    let user = data.into_inner();
    let mut stmt = conn.prepare("SELECT password FROM user WHERE username = ?1").unwrap();
    let stored_password: String = stmt.query_row(params![&user.username], |row| row.get(0)).unwrap_or_default();

    if verify(&user.password, &stored_password).unwrap() {
        id.remember(user.username.clone());
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

async fn register(
    data: web::Json<UserInput>,
    conn: web::Data<Connection>,
) -> Result<HttpResponse, Error> {
    let user = data.into_inner();
    let hashed_pw = hash(&user.password, DEFAULT_COST).unwrap();
    
    match conn.execute(
        "INSERT INTO user (username, password) VALUES (?1, ?2)",
        params![&user.username, &hashed_pw],
    ) {
        Ok(_) => Ok(HttpResponse::Created().finish()),
        Err(_) => Ok(HttpResponse::BadRequest().json("User already exists or another error occurred")),
    }
}

async fn send_message(
    msg: web::Json<Message>,
    ipfs: web::Data<IpfsClient>,
) -> Result<HttpResponse, Error> {
    // ... logic for sending message to IPFS
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ... (оставшаяся часть кода)
}
