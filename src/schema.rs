table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    batches (id) {
        id -> Int4,
        strain_id -> Int4,
        harvest_date -> Nullable<Date>,
        final_test_date -> Nullable<Date>,
        package_date -> Nullable<Date>,
        grower_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    growers (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    strains (id) {
        id -> Int4,
        name -> Varchar,
        species -> Species,
    }
}

joinable!(batches -> growers (grower_id));
joinable!(batches -> strains (strain_id));

allow_tables_to_appear_in_same_query!(
    batches,
    growers,
    strains,
);
