use super::models::{NewStrain, Species, Strain};
use super::schema::strains::dsl::{self, strains};

use diesel::pg::PgConnection;
use diesel::result::Error;
use diesel::sql_types::Integer;
use diesel::{
    sql_query, Connection, ConnectionError, ExpressionMethods, QueryDsl, Queryable, RunQueryDsl,
};
use dotenv::dotenv;
use std::env;

pub struct StrainParams<'a> {
    pub id: Option<i32>,
    pub name: Option<&'a str>,
    pub species: Option<Species>,
}

pub enum StrainField<I = i32, N = String, S = Species> {
    Id(I),
    Name(N),
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
pub trait Deletable<C = PgConnection, E = Error> {
    type Output;
    fn delete(&self, conn: &C) -> Result<Self::Output, E>;
}

/// Trait for retrieving objects
pub trait Retrievable<'a, C = PgConnection, E = Error> {
    type Field;
    type Output;
    fn all(conn: &C) -> Result<Vec<Self::Output>, Error>;
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

impl Retrievable<'_> for Strain {
    type Field = StrainField;
    type Output = Strain;
    /// Retrieves all objects
    fn all(conn: &PgConnection) -> Result<Vec<Strain>, Error> {
        strains.load(conn)
    }

    fn filter(conn: &PgConnection, field: StrainField) -> Result<Vec<Strain>, Error> {
        unimplemented!()
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
            name: "Gaylord OG".to_owned(),
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
        let res = Strain::all(&conn);
        assert!(res.is_ok());
    }
}
