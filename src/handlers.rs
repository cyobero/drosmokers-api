use super::db::*;
use super::models::{Batch, Grower, NewBatch, NewGrower, NewStrain, Species, Strain};
use super::DbPool;
use actix_web::{get, post, web, HttpResponse, Responder};

use diesel::sql_query;

use chrono::NaiveDate;

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Clone)]
struct StrainQuery {
    id: Option<i32>,
    name: Option<String>,
    species: Option<Species>,
}

#[derive(Debug, Deserialize, Clone)]
struct GrowerQuery {
    id: Option<i32>,
    name: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct BatchQuery {
    id: Option<i32>,
    harvest_date: Option<NaiveDate>,
    final_test_date: Option<NaiveDate>,
    package_date: Option<NaiveDate>,
    grower_id: Option<i32>,
    thc_content: Option<f32>,
    cbd_content: Option<f32>,
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
///     `{"200": {"id":30, "name":"High Guys"}}`
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

/// Retrieve a list of all growers or a subset of them that match a given query.
/// Please select one field to query by passing in either `id` or `name` but not both.
///
/// Ex:
///     Request:
///     `$ curl localhost:8008/growers?name=Tegridy%20Farms`
///
///     Response:
///     `$ {"200":[{"id":6,"name":"Tegridy Farms"}]}`
#[get("/growers")]
async fn get_growers(pool: web::Data<DbPool>, query: web::Query<GrowerQuery>) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    web::block(move || match (&query.id, &query.name) {
        (Some(i), _) => Grower::filter(&conn, GrowerField::Id(*i)),
        (_, Some(n)) => Grower::filter(&conn, GrowerField::Name(&&**n)),
        (None, None) => Grower::all(&conn),
    })
    .await
    .map(|res| HttpResponse::Ok().json(json!({ "200": res })))
    .map_err(|e| HttpResponse::InternalServerError().json(json!({"500": e.to_string()})))
}

/// Return an array of batches that match a given query.
///
/// Ex:
///     Request:
///     `$ curl localhost:8008/batches?strain=Fart%20Knocker%20OG&grower=Tegridy%20Farms`
//#[get("/batches")]
//async fn get_batches(pool: web::Data<DbPool>, query: web::Query<BatchQuery>) -> impl Responder {
//let conn = pool.get().expect("Could not get connection.");
//web::block(move || )
//}

#[post("/batches")]
async fn post_new_batch(pool: web::Data<DbPool>, data: web::Json<NewBatch>) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    let batch = data.into_inner();
    web::block(move || batch.create(&conn))
        .await
        .map(|b| HttpResponse::Ok().json(json!({"status code": 200, "data": b})))
        .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))
}

#[get("/strains")]
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

#[get("/strains/")]
async fn get_strains_handler(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get connection.");
    web::block(move || Strain::all(&conn))
        .await
        .map(|strains| web::Json(strains))
        .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))
}
