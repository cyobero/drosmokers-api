use super::schema::{batches, growers, strains, terpenes};

use chrono::NaiveDate;
use diesel::deserialize::FromSql;
use diesel::sql_types::{Date, Float4, Int4, Integer, Nullable, VarChar};
use diesel::{QueryDsl, Queryable, QueryableByName, RunQueryDsl};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Clone, Debug, DbEnum, Deserialize, Serialize, PartialEq)]
pub enum Species {
    Indica,
    Sativa,
    Hybrid,
}

/// Struct used to create new `Strain` object
#[derive(Debug, Deserialize, Serialize, Insertable)]
#[table_name = "strains"]
pub struct NewStrain {
    pub name: String,
    pub species: Species,
}

/// Struct used to create new `Batch` object
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

/// Struct used to create new `Grower` object
#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[table_name = "growers"]
pub struct NewGrower {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[table_name = "terpenes"]
pub struct NewTerpenes {
    pub batch_id: i32,
    pub caryophyllene: Option<f32>,
    pub humulene: Option<f32>,
    pub limonene: Option<f32>,
    pub linalool: Option<f32>,
    pub myrcene: Option<f32>,
    pub pinene: Option<f32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, QueryableByName)]
pub struct BatchResponse<'a> {
    #[sql_type = "VarChar"]
    pub strain: &'a str,

    #[sql_type = "Date"]
    pub harvest_date: Option<NaiveDate>,

    #[sql_type = "Date"]
    pub final_test_date: Option<NaiveDate>,

    #[sql_type = "Date"]
    pub package_date: Option<NaiveDate>,

    #[sql_type = "VarChar"]
    pub grower: &'a str,

    #[sql_type = "Float4"]
    pub thc_content: f32,

    #[sql_type = "Float4"]
    pub cbd_content: f32,
}

/// Struct used for retrieving `Grower` object
#[derive(Clone, Deserialize, Serialize, QueryableByName, Queryable)]
#[table_name = "growers"]
pub struct Grower {
    #[sql_type = "Integer"]
    pub id: i32,

    #[sql_type = "VarChar"]
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, QueryableByName, Queryable)]
#[table_name = "batches"]
pub struct Batch {
    #[sql_type = "Integer"]
    pub id: i32,

    #[sql_type = "Int4"]
    pub strain_id: i32,

    #[sql_type = "Date"]
    pub harvest_date: Option<NaiveDate>,

    #[sql_type = "Date"]
    pub final_test_date: Option<NaiveDate>,

    #[sql_type = "Date"]
    pub package_date: Option<NaiveDate>,

    #[sql_type = "Integer"]
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

#[derive(Debug, Clone, Copy)]
pub struct NewTerpenesBuilder<T = NewTerpenes>(T);

#[derive(Debug, Deserialize, Serialize)]
pub struct NewBatchBuilder<T = NewBatch>(T);

impl NewTerpenesBuilder {
    pub fn batch_id(mut self, id: i32) -> Self {
        self.0.batch_id = id;
        self
    }

    pub fn caryophyllene(mut self, amt: Option<f32>) -> Self {
        self.0.caryophyllene = amt;
        self
    }

    pub fn humulene(mut self, amt: Option<f32>) -> Self {
        self.0.humulene = amt;
        self
    }
    pub fn limonene(mut self, amt: Option<f32>) -> Self {
        self.0.limonene = amt;
        self
    }

    pub fn hinalool(mut self, amt: Option<f32>) -> Self {
        self.0.linalool = amt;
        self
    }

    pub fn myrcene(mut self, amt: Option<f32>) -> Self {
        self.0.myrcene = amt;
        self
    }

    pub fn pinene(mut self, amt: Option<f32>) -> Self {
        self.0.pinene = amt;
        self
    }

    pub fn build(self) -> NewTerpenes {
        NewTerpenes {
            batch_id: self.0.batch_id,
            caryophyllene: self.0.caryophyllene,
            humulene: self.0.humulene,
            limonene: self.0.limonene,
            linalool: self.0.linalool,
            myrcene: self.0.myrcene,
            pinene: self.0.pinene,
        }
    }
}

impl NewBatchBuilder {
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

    pub fn thc_content(mut self, thc: f32) -> Self {
        self.0.thc_content = thc;
        self
    }

    pub fn cbd_content(mut self, cbd: f32) -> Self {
        self.0.cbd_content = cbd;
        self
    }

    pub fn build(self) -> NewBatch {
        NewBatch {
            strain_id: self.0.strain_id,
            harvest_date: self.0.harvest_date,
            final_test_date: self.0.final_test_date,
            package_date: self.0.package_date,
            grower_id: self.0.grower_id,
            thc_content: self.0.thc_content,
            cbd_content: self.0.cbd_content,
        }
    }
}

impl Default for NewTerpenes {
    fn default() -> NewTerpenes {
        NewTerpenes {
            batch_id: -1,
            caryophyllene: None,
            humulene: None,
            limonene: None,
            linalool: None,
            myrcene: None,
            pinene: None,
        }
    }
}

impl NewTerpenes {
    pub fn new() -> Self {
        NewTerpenes::default()
    }

    pub fn builder() -> NewTerpenesBuilder {
        NewTerpenesBuilder(Self::new())
    }
}

impl NewBatch {
    pub fn new() -> Self {
        NewBatch {
            strain_id: -1,
            harvest_date: None,
            final_test_date: None,
            package_date: None,
            grower_id: -1,
            thc_content: 0.0,
            cbd_content: 0.0,
        }
    }

    pub fn builder() -> NewBatchBuilder {
        NewBatchBuilder(Self::new())
    }
}
