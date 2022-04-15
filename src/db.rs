use super::models::{NewStrain, Strain};
use super::schema::strains::dsl::strains;

use diesel::pg::PgConnection;
use diesel::result::Error;
use diesel::{Connection, ConnectionError, ExpressionMethods, QueryDsl, Queryable, RunQueryDsl};
use dotenv::dotenv;
use std::env;

/// A Utility function to establish DB connection
pub fn establish_connection() -> Result<PgConnection, ConnectionError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
}

/// Trait for creating new objects
/// Example:
/// let conn = establish_connection().unwrap();
/// let new = NewStrain {
///         name: "Gaylord OG".to_owned(),
///         species: Species::Indica,
/// };
/// let strain = new.create(&conn);
/// assert!(strain.is_ok());
/// assert_eq!(strain.unwrap().species, Species::Indica);
pub trait Creatable<C = PgConnection, E = Error>
where
    C: Connection,
{
    type Obj;

    fn create(&self, conn: &C) -> Result<Self::Obj, E>;
}

/// Trait for deleting objects
/// Example:
///     let conn = establish_connection().unwrap();
///     let new = NewStrain {
///         name: "Reggie Kush".to_owned(),
///         species: Species::Indica,
///     };
///     let strain = new.create(&conn).unwrap();
///     assert!(strain.delete(&conn).is_ok());
///     let fails = strains.find(strain.id).get_result::<Strain>(&conn);
///     assert!(fails.is_err());
pub trait Deletable<C = PgConnection, E = Error> {
    type Obj;
    fn delete(&self, conn: &C) -> Result<Self::Obj, E>;
}

impl Creatable for NewStrain {
    type Obj = Strain;
    fn create(&self, conn: &PgConnection) -> Result<Strain, Error> {
        diesel::insert_into(strains).values(self).get_result(conn)
    }
}

impl Deletable for Strain {
    type Obj = Strain;
    fn delete(&self, conn: &PgConnection) -> Result<Strain, Error> {
        diesel::delete(strains.find(&self.id)).get_result(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{NewStrain, Species};

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
        assert_eq!(strain.unwrap().species, Species::Indica);
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
}
