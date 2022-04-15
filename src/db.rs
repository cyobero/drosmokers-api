use diesel::pg::PgConnection;
use diesel::{Connection, ConnectionError, ExpressionMethods, QueryDsl, RunQueryDsl};
use dotenv::dotenv;
use std::env;

/// A Utility function to establish DB connection
pub fn establish_connection() -> Result<PgConnection, ConnectionError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
}

#[cfg(test)]
mod tests {
    use super::establish_connection;

    #[test]
    fn connection_established() {
        let conn = establish_connection();
        assert!(conn.is_ok());
    }
}
