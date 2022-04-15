table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    strains (id) {
        id -> Int4,
        name -> Varchar,
        species -> Species,
    }
}
