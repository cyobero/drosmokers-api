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

#[derive(Debug, Deserialize, Clone)]
struct BatchQuery {
    strain_id: Option<i32>,
    grower_id: Option<i32>,
    harvest_date: Option<NaiveDate>,
    final_test_date: Option<NaiveDate>,
    package_date: Option<NaiveDate>,
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
        .map(|g| HttpResponse::Ok().json(json!({ "200": g })))
        .map_err(|e| HttpResponse::InternalServerError().json(json!({"500": e.to_string() })))
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
async fn query_growers(pool: web::Data<DbPool>, query: web::Query<GrowerQuery>) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    web::block(move || match &query.name {
        Some(n) => Grower::filter(&conn, GrowerField::Name(&&**n)),
        None => Grower::all(&conn),
    })
    .await
    .map(|res| HttpResponse::Ok().json(json!({ "200": res })))
    .map_err(|e| HttpResponse::InternalServerError().json(json!({"500": e.to_string()})))
}

/// Get grower by {id}
#[get("/growers/{id}")]
async fn get_grower_by_id(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    web::block(move || growers.find(path.0).first::<Grower>(&conn))
        .await
        .map(|res| HttpResponse::Ok().json(json!({ "200": res })))
        .map_err(|e| HttpResponse::InternalServerError().json(json!({"404": e.to_string() })))
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
        .map(|res| HttpResponse::Ok().json(json!({ "200": res })))
        .map_err(|e| HttpResponse::InternalServerError().json(json!({"500": e.to_string() })))
}

#[post("/batches")]
async fn post_new_batch(pool: web::Data<DbPool>, data: web::Json<NewBatch>) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    let batch = data.into_inner();
    web::block(move || batch.create(&conn))
        .await
        .map(|b| HttpResponse::Ok().json(json!({ "200": b })))
        .map_err(|e| HttpResponse::InternalServerError().json(json!({"500": e.to_string() })))
}

#[get("/strains")]
async fn query_strain(pool: web::Data<DbPool>, query: web::Query<StrainQuery>) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    web::block(move || match (&query.name, &query.species) {
        (Some(n), None) => Strain::filter(&conn, StrainField::Name(&&**n)),
        (None, Some(s)) => Strain::filter(&conn, StrainField::Species(s.clone())),
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
                .json(json!({"status code": 500, "message": e.to_string()}))
        })
}

#[get("/strains/{id}")]
async fn get_strains_by_id(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get connection.");
    web::block(move || {
        sql_query("SELECT * FROM strains WHERE id = $1")
            .bind::<Integer, _>(path.0)
            .load::<Strain>(&conn)
    })
    .await
    .map(|res| HttpResponse::Ok().json(json!({ "200": res })))
    .map_err(|e| HttpResponse::NotFound().json(json!({"404": e.to_string() })))
}
