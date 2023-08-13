use actix_web::{web, HttpResponse, Error, Result};
use actix_identity::Identity;
use serde::Deserialize;
use db::get_user_password;

#[derive(Deserialize)]
pub struct UserInput {
    username: String,
    password: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login")
            .route(web::post().to(login))
    );
    // ... другие роуты
}

pub async fn login(
    id: Identity,
    data: web::Json<UserInput>,
) -> Result<HttpResponse> {
    let user = data.into_inner();
    let stored_password = get_user_password(&user.username)?;

    // ... логика верификации пароля и ответа
}
