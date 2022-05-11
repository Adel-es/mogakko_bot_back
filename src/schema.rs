table! {
    schedules (id) {
        id -> Int4,
        start -> Timestamp,
        end -> Timestamp,
        user_id -> Text,
        title -> Nullable<Text>,
        content -> Nullable<Text>,
    }
}
