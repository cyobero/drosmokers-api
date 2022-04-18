use super::models::{NewStrain, Species, Strain};
use super::schema::strains::dsl::{self, id as sid, name, species, strains};

use diesel::expression::sql_literal::{sql, SqlLiteral};
use diesel::pg::PgConnection;
use diesel::result::Error;
use diesel::sql_types::{Integer, Varchar};
use diesel::{
    sql_query, Connection, ConnectionError, ExpressionMethods, QueryDsl, Queryable, RunQueryDsl,
};
use dotenv::dotenv;
use std::env;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum StrainField<'a, I = i32, N = &'a str, S = Species> {
    Id(I),
    Name(&'a N),
    Species(S),
}

/// A Utility function to establish DB connection
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
pub trait Retrievable<'a, 'n, C = PgConnection, E = Error> {
    type Field;
    type Output;

    /// Retrieve all DB records of this object
    ///
    /// Example:
    /// let conn = establish_connection().unwrap();
    /// let all = Strain::all(&conn);
    /// assert_ne!(all.as_ref().unwrap().len(), 0);
    fn all(conn: &C) -> Result<Vec<Self::Output>, Error>;

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
    fn filter(conn: &C, field: Self::Field) -> Result<Vec<Self::Output>, E>;
}

impl Creatable for NewStrain {
    type Output = Strain;
    fn create(&self, conn: &PgConnection) -> Result<Strain, Error> {
        diesel::insert_into(strains).values(self).get_result(conn)
    }
}

impl Deletable for Strain {
    type Output = Strain;
    fn delete(&self, conn: &PgConnection) -> Result<Strain, Error> {
        diesel::delete(strains.find(&self.id)).get_result(conn)
    }
}

impl<'a> Retrievable<'_, 'a> for Strain {
    type Field = StrainField<'a>;
    type Output = Strain;
    fn all(conn: &PgConnection) -> Result<Vec<Strain>, Error> {
        strains.load(conn)
    }

    fn filter(conn: &PgConnection, field: StrainField) -> Result<Vec<Strain>, Error> {
        match field {
            StrainField::Id(i) => strains.filter(sid.eq(i)).get_results(conn),
            StrainField::Name(n) => strains
                .filter(sql("name ILIKE  ").bind::<Varchar, _>(n))
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
}
