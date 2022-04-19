use super::db::*;
use super::models::{NewBatch, NewStrain, Strain};
use super::DbPool;
use actix_web::{get, post, web, HttpResponse, Responder};

use serde_json::json;

//#[post("/batches")]
//async fn post_new_batch(pool: web::Data<DbPool>, data: web::Json<NewBatch>) -> impl Responder {
//let conn = pool.get().expect("Could not get connection.");
//let batch = data.into_inner();

//unimplemented!()
//}

#[get("/strains/{id}")]
async fn get_strain_id(pool: web::Data<DbPool>, path: web::Path<(i32,)>) -> impl Responder {
    let id = path.into_inner().0;
    let conn = pool.get().expect("Could not get connection.");
    web::block(move || Strain::filter(&conn, StrainField::Id(id)))
        .await
        .map(|s| web::Json(s))
        .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))
}

#[post("/strains")]
async fn post_new_strain(pool: web::Data<DbPool>, data: web::Json<NewStrain>) -> impl Responder {
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
#[get("/strains")]
async fn get_strains_handler(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get connection.");
    web::block(move || Strain::all(&conn))
        .await
        .map(|strains| web::Json(strains))
        .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))
}
