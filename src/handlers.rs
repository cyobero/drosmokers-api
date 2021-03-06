use super::db::*;
use super::models::{Batch, Grower, NewBatch, NewGrower, NewStrain, Species, Strain};
use super::schema::growers::dsl::{growers, id as gid};
use super::schema::strains::dsl::{id as sid, strains};
use super::DbPool;
use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use diesel::sql_query;
use diesel::sql_types::Integer;

use chrono::NaiveDate;

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Clone)]
struct StrainQuery {
    name: Option<String>,
    species: Option<Species>,
}

#[derive(Debug, Deserialize, Clone)]
struct GrowerQuery {
    name: Option<String>,
}

#[get("/growers/{id}/batches")]
async fn get_batches_by_grower_id(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    web::block(move || Batch::filter(&conn, BatchField::GrowerID(path.0)))
        .await
        .map(|res| match res.len() {
            0 => HttpResponse::NotFound().json(json!({"404": "No Batches Found"})),
            _ => HttpResponse::Ok().json(json!({ "data": res, "status code": 200 })),
        })
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .json(json!({"message": e.to_string(), "status code": 500 }))
        })
}

/// Make a POST request to create a new `Grower` object.
///
/// EX:
///     Request:
///     `$ curl -X POST \
///      $ -H "Content-Type: application/json" \
///      $ -d '{"name": "Tegridy"}'
///      $ localhost:8008/growers`
///
///     Response:
///     `{"201": {"id":30, "name":"High Guys"}}`
#[post("/growers")]
async fn post_new_grower(pool: web::Data<DbPool>, data: web::Json<NewGrower>) -> impl Responder {
    let grower = data.into_inner();
    let conn = pool.get().expect("Could not get connection.");
    web::block(move || grower.create(&conn))
        .await
        .map(|g| HttpResponse::Ok().json(json!({ "data": g, "status code": 201 })))
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .json(json!({"message": e.to_string(), "status code": 500 }))
        })
}

/// Retrieve a list of all growers or a subset of them that match a given query.
/// Please select one field to query by passing in either `id` or `name` but not both.
///
/// Ex:
///     Request:
///     `$ curl localhost:8008/growers?name=Tegridy%20Farms`
///
///     Response:
///     `$ {"200":[{"id":6,"name":"Tegridy Farms"}, {"id":42,"name":"Stuco"}]}`
#[get("/growers")]
async fn query_growers(pool: web::Data<DbPool>, query: web::Query<GrowerQuery>) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    web::block(move || match query.0.name {
        Some(n) => Grower::filter(&conn, GrowerField::Name(n)),
        None => Grower::all(&conn),
    })
    .await
    .map(|res| match res.len() {
        0 => HttpResponse::NotFound()
            .json(json!({"message": "No Growers Found", "status code": 404 })),
        _ => HttpResponse::Ok().json(json!({ "data": res, "Status Code": 200 })),
    })
    .map_err(|e| {
        HttpResponse::InternalServerError()
            .json(json!({"message": e.to_string(), "Status Code": 500 }))
    })
}

/// Get grower by {id}
#[get("/growers/{id}")]
async fn get_grower_by_id(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    web::block(move || growers.find(path.0).first::<Grower>(&conn))
        .await
        .map(|res| HttpResponse::Ok().json(json!({ "data": res, "status code": 200 })))
        .map_err(|e| {
            HttpResponse::NotFound().json(json!({"message": e.to_string(), "status code": 404 }))
        })
}

/// Return an array of batches that match a given query.
///
/// Ex:
///     Request:
///     `$ curl localhost:8008/batches?strain_id=15&grower_id=3`
//#[get("/batches")]
//async fn get_batches(pool: web::Data<DbPool>, query: web::Query<BatchQuery>) -> impl Responder {
//let conn = pool.get().expect("Could not get connection.");
//web::block(move || match query.0 {})
//}
#[get("/batches")]
async fn get_all_batches(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    web::block(move || Batch::all(&conn))
        .await
        .map(|res| HttpResponse::Ok().json(json!({ "data": res, "status code": 200 })))
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .json(json!({"message": e.to_string(), "status code": 500 }))
        })
}

#[post("/batches")]
async fn post_new_batch(pool: web::Data<DbPool>, data: web::Json<NewBatch>) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    let batch = data.into_inner();
    web::block(move || batch.create(&conn))
        .await
        .map(|b| HttpResponse::Ok().json(json!({ "data": b, "status code": 201 })))
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .json(json!({"message": e.to_string(), "status code": 500 }))
        })
}

#[get("/strains")]
async fn query_strain(pool: web::Data<DbPool>, query: web::Query<StrainQuery>) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    web::block(move || match (query.0.name, query.0.species) {
        (Some(n), None) => Strain::filter(&conn, StrainField::Name(n)),
        (None, Some(s)) => Strain::filter(&conn, StrainField::Species(s.clone())),
        _ => Strain::all(&conn),
    })
    .await
    .map(|res| HttpResponse::Ok().json(json!({ "data": res, "status code": 200 })))
    .map_err(|e| {
        HttpResponse::InternalServerError()
            .json(json!({"message": e.to_string(), "status code": 500 }))
    })
}

#[post("/strains")]
async fn post_new_strain(pool: web::Data<DbPool>, data: web::Json<NewStrain>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get connection.");
    let strain = data.into_inner();
    web::block(move || strain.create(&conn))
        .await
        .map(|s| HttpResponse::Ok().json(json!({ "data": s, "status code": 201 })))
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .json(json!({"status code": 500, "message": e.to_string()}))
        })
}

#[get("/strains/{id}")]
async fn get_strains_by_id(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get connection.");
    web::block(move || strains.find(path.0).first::<Strain>(&conn))
        .await
        .map(|res| HttpResponse::Ok().json(json!({ "data": res, "status code": 200 })))
        .map_err(|e| {
            HttpResponse::NotFound().json(json!({"message": e.to_string(), "status code": 404 }))
        })
}

#[get("/strains/{strain_id}/batches")]
async fn get_batches_by_strain_id(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    web::block(move || Batch::filter(&conn, BatchField::StrainID(path.0)))
        .await
        .map(|res| match res.len() {
            0 => HttpResponse::NotFound().json(json!({
                "status code": 404,
                "message": "NotFound"
            })),
            _ => HttpResponse::Ok().json(json!({ "data": res, "status code": 200 })),
        })
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .json(json!({"message": e.to_string(), "status code": 500 }))
        })
}
