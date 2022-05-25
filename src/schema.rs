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
