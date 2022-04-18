use super::db::*;
use super::models::{NewStrain, Strain};
use super::DbPool;
use actix_web::{get, post, web, HttpResponse, Responder};

use serde_json::json;

#[get("/strains")]
async fn get_strains_handler(pool: web::Data<DbPool>) -> Result<impl Responder, HttpResponse> {
    let conn = pool.get().expect("Couldn't get connection.");
    web::block(move || Strain::all(&conn))
        .await
        .map(|strains| web::Json(strains))
        .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))
}

#[post("/strains")]
async fn post_new_strain(
    pool: web::Data<DbPool>,
    data: web::Json<NewStrain>,
) -> Result<impl Responder, HttpResponse> {
    let conn = pool.get().expect("Couldn't get connection.");
    let strain = data.into_inner();
    web::block(move || strain.create(&conn))
        .await
        .map(|s| HttpResponse::Ok().json(json!({"status code": 200, "data": s})))
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .json(json!({"status code": 400, "message": e.to_string()}))
        })
}
