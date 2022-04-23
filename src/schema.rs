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
        thc_content -> Float4,
        cbd_content -> Float4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    growers (id) {
        id -> Int4,
        name -> Varchar,
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

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    terpenes (id) {
        id -> Int4,
        batch_id -> Int4,
        caryophyllene -> Nullable<Float4>,
        humulene -> Nullable<Float4>,
        limonene -> Nullable<Float4>,
        linalool -> Nullable<Float4>,
        myrcene -> Nullable<Float4>,
        pinene -> Nullable<Float4>,
    }
}

joinable!(batches -> growers (grower_id));
joinable!(batches -> strains (strain_id));
joinable!(terpenes -> batches (batch_id));

allow_tables_to_appear_in_same_query!(
    batches,
    growers,
    strains,
    terpenes,
);
