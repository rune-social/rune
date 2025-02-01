// @generated automatically by Diesel CLI.

diesel::table! {
    audits (id) {
        id -> Int8,
        token_id -> Int8,
        #[max_length = 255]
        event -> Varchar,
        detail -> Nullable<Json>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    configs (id) {
        id -> Int8,
        #[max_length = 255]
        key -> Varchar,
        value -> Text,
    }
}

diesel::table! {
    notes (id) {
        id -> Int8,
        user_id -> Int8,
        content -> Nullable<Text>,
        reply_of_note_id -> Nullable<Int8>,
        renote_of_note_id -> Nullable<Int8>,
        edit_of_note_id -> Nullable<Int8>,
        created_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    tokens (id) {
        id -> Int8,
        user_id -> Int8,
        created_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        username -> Varchar,
        #[max_length = 255]
        hash -> Varchar,
        #[max_length = 255]
        totp -> Nullable<Varchar>,
        bot_owner_user_id -> Nullable<Int8>,
        #[max_length = 255]
        display_name -> Nullable<Varchar>,
        bio -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(audits -> tokens (token_id));
diesel::joinable!(notes -> users (user_id));
diesel::joinable!(tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    audits, configs, notes, tokens, users,
);
