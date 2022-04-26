use super::db::*;
use super::models::{Batch, Grower, NewBatch, NewGrower, NewStrain, Species, Strain};
use super::DbPool;
use actix_web::{get, post, web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[post("/growers")]
async fn post_new_grower(pool: web::Data<DbPool>, data: web::Json<NewGrower>) -> impl Responder {
    let grower = data.into_inner();
    let conn = pool.get().expect("Could not get connection.");
    web::block(move || grower.create(&conn))
        .await
        .map(|g| HttpResponse::Ok().json(json!({"status code": 200, "data": g})))
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .json(json!({"status code": 500, "message": e.to_string()}))
        })
}

#[get("/growers")]
async fn get_all_growers(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    web::block(move || Grower::all(&conn))
        .await
        .map(|g| web::Json(g))
        .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))
}

#[get("/batches")]
async fn get_all_batches(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    web::block(move || Batch::all(&conn))
        .await
        .map(|b| web::Json(b))
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .json(json!({"status code": 500, "message": e.to_string()}))
        })
}

#[post("/batches")]
async fn post_new_batch(pool: web::Data<DbPool>, data: web::Json<NewBatch>) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    let batch = data.into_inner();
    web::block(move || batch.create(&conn))
        .await
        .map(|b| HttpResponse::Ok().json(json!({"status code": 200, "data": b})))
        .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))
}

#[derive(Debug, Deserialize, Clone)]
struct StrainQuery {
    id: Option<i32>,
    name: Option<String>,
    species: Option<Species>,
}

#[get("/strains/")]
async fn query_strain(pool: web::Data<DbPool>, query: web::Query<StrainQuery>) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    web::block(move || match (&query.id, &query.name, &query.species) {
        (Some(i), _, _) => Strain::filter(&conn, StrainField::Id(*i)),
        (_, Some(n), _) => Strain::filter(&conn, StrainField::Name(&&**n)),
        (_, _, Some(s)) => Strain::filter(&conn, StrainField::Species(s.clone())),
        _ => Strain::all(&conn),
    })
    .await
    .map(|res| HttpResponse::Ok().json(json!({ "200": res })))
    .map_err(|e| HttpResponse::InternalServerError().json(json!({"500": e.to_string()})))
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
