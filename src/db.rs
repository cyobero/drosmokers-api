use super::models::*;
use super::schema::batches::dsl::{
    batches, cbd_content, final_test_date, grower_id, harvest_date, id as bid, package_date,
    strain_id, thc_content,
};
use super::schema::growers::dsl::{growers, id as gid, name as grower_name};
use super::schema::strains::dsl::{id as sid, name, species, strains};
use super::schema::terpenes::dsl::*;

use chrono::NaiveDate;
use diesel::expression::sql_literal::sql;
use diesel::pg::PgConnection;
use diesel::result::Error;
use diesel::sql_types::{Date, Integer, VarChar, Varchar};
use diesel::{sql_query, Connection, ConnectionError, ExpressionMethods, QueryDsl, RunQueryDsl};
use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub enum BatchField<'b> {
    Id(i32),
    Strain(&'b str),
    StrainID(i32),
    HarvestDate(NaiveDate),
    FinalTestDate(NaiveDate),
    PackageDate(NaiveDate),
    GrowerID(i32),
    Grower(&'b str),
    THCContent(f32),
    CBDContent(f32),
}

#[derive(Debug, Clone, Copy)]
pub enum GrowerField<'g, I = i32, N = &'g str> {
    Id(I),
    Name(&'g N),
}

#[derive(Debug, Clone, Copy)]
pub enum StrainField<'s, I = i32, N = &'s str, S = Species> {
    Id(I),
    Name(&'s N),
    Species(S),
}

/// A utility function to establish DB connection
/// Example:
/// let conn = establish_connection();
/// assert!(conn.is_ok());
pub fn establish_connection() -> Result<PgConnection, ConnectionError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
}

/// Trait for creating new objects
/// Example:
/// let conn = establish_connection().unwrap();
/// let new = NewStrain {
///     name: "Gaylord OG".to_owned(),
///     species: Species::Indica,
/// };
/// let strain = new.create(&conn);
/// assert!(strain.is_ok());
/// assert_eq!(strain.unwrap().species, Species::Indica);
pub trait Creatable<C = PgConnection, E = Error>
where
    C: Connection,
{
    type Output;

    fn create(&self, conn: &C) -> Result<Self::Output, E>;
}

/// Trait for deleting objects
pub trait Deletable<C = PgConnection, E = Error> {
    type Output;
    /// Remove database record of this instance
    ///
    /// Example:
    /// let conn = establish_connection().unwrap();
    /// let new = NewStrain {
    ///     name: "Reggie Kush".to_owned(),
    ///     species: Species::Indica,
    /// };
    /// let strain = new.create(&conn).unwrap();
    /// assert!(strain.delete(&conn).is_ok());
    /// let fails = strains.find(strain.id).get_result::<Strain>(&conn);
    /// assert!(fails.is_err());
    fn delete(&self, conn: &C) -> Result<Self::Output, E>;
}

/// Trait for retrieving objects
pub trait Retrievable<'a, Output = Self, C = PgConnection, E = Error> {
    type Field;

    /// Retrieve all DB records of this object
    ///
    /// Example:
    /// let conn = establish_connection().unwrap();
    /// let all = Strain::all(&conn);
    /// assert_ne!(all.as_ref().unwrap().len(), 0);
    fn all(conn: &C) -> Result<Vec<Output>, Error>;

    /// Retrieve a collection of objects that match a specified criteria
    ///
    /// Example
    /// use super::StrainField;
    /// use crate::models::{Species, Species::*};
    ///
    /// let conn = establish_connection().unwrap();
    /// let filtered_by_id = Strain::filter(&conn, StrainField::Id(3)).unwrap();
    /// let indicas = Strain::filter(&conn, StrainField::Species(Indica)).unwrap();
    /// let cake = Strain::filter(&conn, StrainField::Name(&"wedding cake")).unwrap();
    ///
    /// assert_eq!(filtered_by_id[0].id, 3);
    /// assert_eq!(indicas[2].species, Species::Indica);
    /// assert_eq!(cake[0].name, "Wedding Cake");
    fn filter(conn: &C, field: Self::Field) -> Result<Vec<Output>, E>;
}

impl Creatable for NewTerpenes {
    type Output = Terpenes;
    fn create(&self, conn: &PgConnection) -> Result<Terpenes, Error> {
        diesel::insert_into(terpenes).values(self).get_result(conn)
    }
}

impl Creatable for NewGrower {
    type Output = Grower;
    fn create(&self, conn: &PgConnection) -> Result<Grower, Error> {
        diesel::insert_into(growers).values(self).get_result(conn)
    }
}

impl Creatable for NewBatch {
    type Output = Batch;
    fn create(&self, conn: &PgConnection) -> Result<Batch, Error> {
        diesel::insert_into(batches).values(self).get_result(conn)
    }
}

impl Creatable for NewStrain {
    type Output = Strain;
    fn create(&self, conn: &PgConnection) -> Result<Strain, Error> {
        diesel::insert_into(strains).values(self).get_result(conn)
    }
}

impl Deletable for Grower {
    type Output = Grower;
    fn delete(&self, conn: &PgConnection) -> Result<Grower, Error> {
        diesel::delete(growers.find(&self.id)).get_result(conn)
    }
}

impl Deletable for Batch {
    type Output = Batch;
    fn delete(&self, conn: &PgConnection) -> Result<Batch, Error> {
        diesel::delete(batches.find(&self.id)).get_result(conn)
    }
}

impl Deletable for Strain {
    type Output = Strain;
    fn delete(&self, conn: &PgConnection) -> Result<Strain, Error> {
        diesel::delete(strains.find(&self.id)).get_result(conn)
    }
}

impl<'b> Retrievable<'b, BatchResponse> for Batch {
    type Field = BatchField<'b>;
    fn all(conn: &PgConnection) -> Result<Vec<BatchResponse>, Error> {
        sql_query(
            "SELECT s.name as strain, b.harvest_date, b.final_test_date, b.package_date,
             g.name as grower, b.thc_content, b.cbd_content FROM batches b INNER JOIN
             strains s ON b.strain_id = s.id INNER JOIN growers g ON b.grower_id = g.id",
        )
        .get_results(conn)
    }

    fn filter(conn: &PgConnection, field: BatchField) -> Result<Vec<BatchResponse>, Error> {
        let stmt = "SELECT s.name as strain, b.harvest_date, b.final_test_date, b.package_date,
             g.name as grower, b.thc_content, b.cbd_content FROM batches b INNER JOIN
             strains s ON b.strain_id = s.id INNER JOIN growers g ON b.grower_id = g.id "
            .to_owned();

        match field {
            BatchField::StrainID(_sid) => sql_query(stmt + " WHERE b.strain_id = 3 ")
                .bind::<Integer, _>(_sid)
                .get_results(conn),

            BatchField::Strain(s) => sql_query(stmt + " WHERE s.name = $1 ")
                .bind::<VarChar, _>(s)
                .get_results(conn),

            BatchField::HarvestDate(h) => sql_query(stmt + "WHERE harvest_date = '$1'")
                .bind::<Date, _>(h)
                .get_results(conn),

            BatchField::FinalTestDate(h) => sql_query(stmt + "WHERE final_test_date = '$1'")
                .bind::<Date, _>(h)
                .get_results(conn),

            BatchField::GrowerID(g) => sql_query(stmt + "WHERE g.id = $1")
                .bind::<Integer, _>(g)
                .get_results(conn),

            BatchField::Grower(gr) => sql_query(stmt + "WHERE g.name = $1")
                .bind::<Varchar, _>(gr)
                .get_results(conn),

            _ => Self::all(conn),
        }
    }
}

impl<'a> Retrievable<'a> for Grower {
    type Field = GrowerField<'a>;
    fn all(conn: &PgConnection) -> Result<Vec<Grower>, Error> {
        growers.load(conn)
    }

    fn filter(conn: &PgConnection, field: GrowerField) -> Result<Vec<Grower>, Error> {
        match field {
            GrowerField::Id(i) => growers.filter(gid.eq(i)).get_results(conn),
            GrowerField::Name(n) => growers
                .filter(sql("name ILIKE ").bind::<VarChar, _>(n))
                .get_results(conn),
        }
    }
}

impl<'a> Retrievable<'a> for Strain {
    type Field = StrainField<'a>;
    fn all(conn: &PgConnection) -> Result<Vec<Strain>, Error> {
        strains.load(conn)
    }

    fn filter(conn: &PgConnection, field: StrainField) -> Result<Vec<Strain>, Error> {
        match field {
            StrainField::Id(i) => strains.filter(sid.eq(i)).get_results(conn),
            StrainField::Name(n) => strains
                .filter(sql("name ILIKE  ").bind::<VarChar, _>(n))
                .get_results(conn),
            StrainField::Species(s) => strains.filter(species.eq(s)).get_results(conn),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{NewStrain, Species, Strain};

    #[test]
    fn connection_established() {
        let conn = establish_connection();
        assert!(conn.is_ok());
    }

    #[test]
    fn new_strain_created() {
        let conn = establish_connection().unwrap();
        let new = NewStrain {
            name: "Test OG".to_owned(),
            species: Species::Indica,
        };
        let strain = new.create(&conn);

        assert!(strain.is_ok());
        assert_eq!(strain.as_ref().unwrap().species, Species::Indica);
        diesel::delete(strains.find(strain.as_ref().unwrap().id))
            .get_result::<Strain>(&conn)
            .unwrap();
    }

    #[test]
    fn strain_deleted() {
        let conn = establish_connection().unwrap();
        let new = NewStrain {
            name: "Reggie Kush".to_owned(),
            species: Species::Indica,
        };
        let strain = new.create(&conn).unwrap();
        assert!(strain.delete(&conn).is_ok());
        let fails = strains.find(strain.id).get_result::<Strain>(&conn);
        assert!(fails.is_err());
    }

    #[test]
    fn all_strains_retrieved() {
        let conn = establish_connection().unwrap();
        let all = Strain::all(&conn);
        assert_ne!(all.as_ref().unwrap().len(), 0);
    }

    #[test]
    fn strain_filtered_by_id() {
        use super::StrainField;
        use crate::models::{Species, Species::*};
        let conn = establish_connection().unwrap();
        let filtered_by_id = Strain::filter(&conn, StrainField::Id(3)).unwrap();
        let indicas = Strain::filter(&conn, StrainField::Species(Indica)).unwrap();
        assert_eq!(filtered_by_id[0].id, 3);
        assert_eq!(indicas[2].species, Species::Indica);
    }

    #[test]
    fn strain_filtered_by_species() {
        use super::StrainField;
        use crate::models::{Species, Species::*};
        let conn = establish_connection().unwrap();
        let indicas = Strain::filter(&conn, StrainField::Species(Indica)).unwrap();
        assert_eq!(indicas[2].species, Species::Indica);
    }

    #[test]
    fn strain_filtered_by_name() {
        use super::StrainField;
        let conn = establish_connection().unwrap();
        let res = Strain::filter(&conn, StrainField::Name(&"gaylord oG")).unwrap();
        assert_eq!(res[0].name, "Gaylord OG");
    }

    #[test]
    fn new_batch_created() {
        use crate::models::NewBatch;
        let conn = establish_connection().unwrap();
        let batch = NewBatch::builder()
            .strain_id(3)
            .grower_id(3)
            .thc_content(22.9)
            .cbd_content(0.2)
            .build()
            .create(&conn);

        assert_eq!(batch.unwrap().strain_id, 3);
    }

    #[test]
    fn batch_deleted() {
        use super::Deletable;
        let conn = establish_connection().unwrap();
        let batch = NewBatch::builder()
            .strain_id(1)
            .grower_id(3)
            .thc_content(32.9)
            .cbd_content(1.2)
            .build()
            .create(&conn)
            .unwrap();

        assert!(batch.delete(&conn).is_ok());
    }

    #[test]
    fn new_grower_created() {
        use super::Creatable;
        let conn = establish_connection().unwrap();
        let new = NewGrower {
            name: "Tegridy Farms".to_string(),
        };
        let grower = new.create(&conn);
    }

    #[test]
    fn grower_retrieved_by_name() {
        use super::Retrievable;
        let conn = establish_connection().unwrap();
        let tegridy = Grower::filter(&conn, GrowerField::Name(&"Tegridy Farms")).unwrap();
        assert_eq!(tegridy[0].name, "Tegridy Farms");
    }

    #[test]
    fn all_growers_retrieved() {
        use super::Retrievable;
        let conn = establish_connection().unwrap();
        let _growers = Grower::all(&conn);
        assert!(_growers.is_ok());
    }

    #[test]
    fn all_batches_retrieved() {
        use super::Retrievable;
        let conn = establish_connection().unwrap();
        let all = Batch::all(&conn).unwrap();
        assert_ne!(all.len(), 0);
    }

    #[test]
    fn batch_filtered_by_strain_name() {
        use super::Retrievable;
        let conn = establish_connection().unwrap();
        let res = Batch::filter(&conn, BatchField::Strain("Blackwater OG")).unwrap();
        assert_eq!(res[0].strain, "Blackwater OG");
    }

    #[test]
    fn batch_filtered_by_strain_id() {
        use super::Retrievable;
        let conn = establish_connection().unwrap();
        let res = Batch::filter(&conn, BatchField::StrainID(3)).unwrap();
        assert_eq!(res[0].strain, "Blackwater OG".to_owned());
    }

    #[test]
    fn batch_filtered_by_grower_id() {
        let conn = establish_connection().unwrap();
        let res = Batch::filter(&conn, BatchField::GrowerID(3)).unwrap();
        assert_eq!(res[0].grower, "Summa");
    }
    #[test]
    fn batch_filtered_by_grower_name() {
        let conn = establish_connection().unwrap();
        let res = Batch::filter(&conn, BatchField::Grower("Summa")).unwrap();
        assert_eq!(res[0].grower, "Summa");
    }
}
