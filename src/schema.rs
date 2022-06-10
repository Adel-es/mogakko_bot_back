table! {
    schedules (id) {
        id -> Int4,
        start -> Timestamp,
        end -> Timestamp,
        user_id -> Text,
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

allow_tables_to_appear_in_same_query!(
    schedules,
    users,
);
