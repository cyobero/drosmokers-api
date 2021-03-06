#[macro_use]
extern crate diesel;

mod db;
pub mod handlers;
pub mod models;
pub mod schema;

use self::handlers::*;

use actix_web::{web, App, HttpServer};

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub mod exports {
    pub use crate::models::SpeciesMapping as Species;
}

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Could not create pool.");

    let addrress = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8008);
    println!("Serving at {:?}", addrress);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(get_strains_by_id)
            .service(post_new_strain)
            .service(query_strain)
            .service(post_new_batch)
            .service(get_grower_by_id)
            .service(query_growers)
            .service(post_new_grower)
            .service(get_all_batches)
            .service(get_batches_by_strain_id)
            .service(get_batches_by_grower_id)
    })
    .bind(addrress)?
    .run()
    .await
}
