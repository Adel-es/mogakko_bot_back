table! {
    schedules (id) {
        id -> Int4,
        start -> Timestamp,
        end -> Timestamp,
        user_id -> Int4,
        title -> Text,
        content -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        pw -> Text,
        nickname -> Text,
        discord_id -> Text,
        created -> Timestamp,
        memo -> Text,
    }
}

joinable!(schedules -> users (user_id));

allow_tables_to_appear_in_same_query!(
    schedules,
    users,
);
