#[macro_use]
extern crate diesel;

pub mod db;
pub mod models;
pub mod schema;

pub mod exports {
    pub use crate::models::SpeciesMapping as Species;
}

fn main() {
    println!("Hello, world!");
}
