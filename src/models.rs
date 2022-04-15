use super::schema::strains;

use diesel::{QueryDsl, RunQueryDsl};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, DbEnum, Deserialize, Serialize)]
pub enum Species {
    Indica,
    Sativa,
    Hybrid,
}

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[table_name = "strains"]
pub struct NewStrain {
    pub name: String,
    pub species: Species,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Strain {
    id: i32,
    pub name: String,
    pub species: Species,
}
