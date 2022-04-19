use super::schema::{batches, strains};

use chrono::NaiveDate;
use diesel::deserialize::FromSql;
use diesel::sql_types::{Date, Float4, Int4, Integer, Nullable, VarChar};
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
#[table_name = "batches"]
pub struct Batch {
    #[sql_type = "Int4"]
    pub id: i32,

    #[sql_type = "Int4"]
    pub strain_id: i32,

    #[sql_type = "Date"]
    pub harvest_date: Option<NaiveDate>,

    #[sql_type = "Date"]
    pub final_test_date: Option<NaiveDate>,

    #[sql_type = "Date"]
    pub package_date: Option<NaiveDate>,

    #[sql_type = "Int4"]
    pub grower_id: i32,

    #[sql_type = "Float4"]
    pub thc_content: f32,

    #[sql_type = "Float4"]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct BatchBuilder(Batch);

impl BatchBuilder {
    pub fn strain_id(mut self, id: i32) -> Self {
        self.0.strain_id = id;
        self
    }

    pub fn harvest_date(mut self, date: Option<NaiveDate>) -> Self {
        self.0.harvest_date = date;
        self
    }

    pub fn final_test_date(mut self, date: Option<NaiveDate>) -> Self {
        self.0.final_test_date = date;
        self
    }

    pub fn package_date(mut self, date: Option<NaiveDate>) -> Self {
        self.0.package_date = date;
        self
    }

    pub fn grower_id(mut self, id: i32) -> Self {
        self.0.grower_id = id;
        self
    }

    pub fn thc_content(mut self, amt: f32) -> Self {
        self.0.thc_content = amt;
        self
    }

    pub fn cbd_content(mut self, amt: f32) -> Self {
        self.0.cbd_content = amt;
        self
    }
}
