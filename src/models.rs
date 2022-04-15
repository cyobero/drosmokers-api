use super::schema::strains;

use diesel::sql_types::{Integer, VarChar};
use diesel::{QueryDsl, Queryable, QueryableByName, RunQueryDsl};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, DbEnum, Deserialize, Serialize, PartialEq)]
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

#[derive(Debug, Deserialize, Serialize, QueryableByName, Queryable)]
pub struct Strain {
    #[sql_type = "Integer"]
    id: i32,
    #[sql_type = "VarChar"]
    pub name: String,
    #[sql_type = "SpeciesMapping"]
    pub species: Species,
}
