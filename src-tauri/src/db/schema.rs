// @generated automatically by Diesel CLI.

diesel::table! {
    devices (id) {
        id -> Integer,
        unique_id -> Text,
        name -> Text,
        certificate -> Binary,
        is_trusted -> Bool,
    }
}

diesel::table! {
    documents (id) {
        id -> Integer,
        title -> Text,
        path -> Text,
        kind -> Text,
        body -> Text,
    }
}

diesel::table! {
    events (id) {
        id -> Integer,
        creator_id -> Integer,
        creator_counter -> Integer,
        vector_clock -> Text,
        document_path -> Text,
        kind -> Text,
        timestamp -> Timestamp,
        data -> Text,
    }
}

diesel::table! {
    order_of_events (rowid) {
        rowid -> Integer,
        order -> Integer,
        document_path -> Text,
        event_id -> Integer,
    }
}

diesel::table! {
    private_keys (rowid) {
        rowid -> Integer,
        private_key -> Nullable<Binary>,
    }
}

diesel::table! {
    vector_clock (device_id) {
        device_id -> Integer,
        counter -> Integer,
    }
}

diesel::joinable!(events -> devices (creator_id));
diesel::joinable!(order_of_events -> events (event_id));
diesel::joinable!(vector_clock -> devices (device_id));

diesel::allow_tables_to_appear_in_same_query!(
    devices,
    documents,
    events,
    order_of_events,
    private_keys,
    vector_clock,
);
