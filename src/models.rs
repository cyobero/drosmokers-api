use super::schema::{batches, strains};

use chrono::NaiveDate;
use diesel::sql_types::{Integer, VarChar};
use diesel::{QueryDsl, Queryable, QueryableByName, RunQueryDsl};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

use std::fmt;

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

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[table_name = "batches"]
pub struct NewBatch {
    pub strain_id: i32,
    pub harvest_date: Option<NaiveDate>,
    pub final_test_date: Option<NaiveDate>,
    pub package_date: Option<NaiveDate>,
    pub grower_id: i32,
    pub thc_content: f32,
    pub cbd_content: f32,
}

#[derive(Debug, Deserialize, Serialize, QueryableByName, Queryable)]
pub struct Strain {
    #[sql_type = "Integer"]
    pub id: i32,

    #[sql_type = "VarChar"]
    pub name: String,

    #[sql_type = "SpeciesMapping"]
    pub species: Species,
}

impl fmt::Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Species::Indica => write!(f, "indica"),
            Species::Sativa => write!(f, "sativa"),
            Species::Hybrid => write!(f, "hybrid"),
        }
    }
}
