use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection};
use ipfs_api::IpfsClient;
use db::save_message_hash;
use ring::signature::UnparsedPublicKey;
use log::warn;

#[derive(Deserialize)]
pub struct SendMessage {
    from: String,
    to: String,
    content: String,
    signature: String,  // Допустим, что подпись представлена в виде base64 строки
}

#[derive(Serialize)]
pub struct SendMessageResponse {
    ipfs_hash: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/send_message")
            .route(web::post().to(send_message))
    );
}

pub async fn send_message(
    msg: web::Json<SendMessage>,
    ipfs: web::Data<IpfsClient>,
    conn: web::Data<Connection>,
) -> Result<HttpResponse> {
    let public_key = match get_public_key_for_user(&conn, &msg.from) {
        Ok(pk) => pk,
        Err(_) => {
            warn!("Failed to retrieve the public key for user {}", &msg.from);
            return Err(actix_web::error::ErrorInternalServerError("Failed to retrieve the public key"));
        }
    };
    
    let signature_bytes = base64::decode(&msg.signature).expect("Failed to decode base64 signature");
    
    let is_valid_signature = check_signature(&public_key, &msg.content, &signature_bytes);
    if !is_valid_signature {
        warn!("Invalid signature received from {}", &msg.from);
        return Err(actix_web::error::ErrorBadRequest("Invalid signature"));
    }

    let add_response = ipfs.add(&msg.content).await;
    match add_response {
        Ok(res) => {
            let ipfs_hash = res.hash;

            if let Err(e) = save_message_hash(&msg.from, &msg.to, &ipfs_hash, &conn) {
                return Err(actix_web::error::ErrorInternalServerError(e));
            }

            Ok(HttpResponse::Ok().json(SendMessageResponse { ipfs_hash }))
        }
        Err(_) => Err(actix_web::error::ErrorInternalServerError("Failed to send message to IPFS"))
    }
}

fn get_public_key_for_user(conn: &Connection, user: &str) -> Result<String, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT public_key FROM users WHERE username = ?1")?;
    let public_key: String = stmt.query_row(params![user], |row| row.get(0))?;
    Ok(public_key)
}

fn check_signature(public_key: &str, content: &str, signature: &[u8]) -> bool {
    // Допустим, что публичные ключи хранятся в формате PEM или DER. 
    // Здесь мы предполагаем простой пример с использованием ring и ED25519
    let public_key_bytes = base64::decode(public_key).expect("Failed to decode base64 public key");
    
    let verification_algorithm = &ring::signature::ED25519;
    let unparsed_pk = UnparsedPublicKey::new(verification_algorithm, &public_key_bytes);
    
    unparsed_pk.verify(content.as_bytes(), signature).is_ok()
}
