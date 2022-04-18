use super::db::*;
use super::models::Strain;
use super::DbPool;
use actix_web::{get, post, web, HttpResponse, Responder, Result};

use serde_json::json;

use std::error::Error;

#[get("/strains")]
async fn get_strains_handler(pool: web::Data<DbPool>) -> Result<impl Responder, HttpResponse> {
    let conn = pool.get().expect("Couldn't get connection.");
    web::block(move || Strain::all(&conn))
        .await
        .map(|strains| web::Json(strains))
        .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))
}
