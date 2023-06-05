// @generated automatically by Diesel CLI.

diesel::table! {
    search_requests (id) {
        id -> Uuid,
        api_key -> Text,
        search_string -> Text,
        successful -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        api_key -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(search_requests, users,);
