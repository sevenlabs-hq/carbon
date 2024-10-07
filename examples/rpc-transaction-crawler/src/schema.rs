// @generated automatically by Diesel CLI.

diesel::table! {
    activities (activity_id) {
        activity_id -> Int4,
        #[max_length = 255]
        signature -> Varchar,
    }
}

diesel::table! {
    vote_entries (voter_id, vote_id) {
        #[max_length = 44]
        voter_id -> Varchar,
        #[max_length = 44]
        vote_id -> Varchar,
        choice -> Bool,
    }
}

diesel::table! {
    votes (vote_id) {
        #[max_length = 44]
        vote_id -> Varchar,
        #[max_length = 44]
        authority -> Varchar,
        yes -> Nullable<Int4>,
        no -> Nullable<Int4>,
    }
}

diesel::joinable!(vote_entries -> votes (vote_id));

diesel::allow_tables_to_appear_in_same_query!(
    activities,
    vote_entries,
    votes,
);
