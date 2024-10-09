// @generated automatically by Diesel CLI.

diesel::table! {
    activities (id) {
        id -> Int4,
        #[max_length = 255]
        activity_type -> Varchar,
        #[max_length = 255]
        signature -> Varchar,
    }
}
