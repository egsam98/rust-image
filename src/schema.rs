table! {
    images (id) {
        id -> Integer,
        title -> Text,
        content_type -> Text,
        bytes -> Binary,
        preview_bytes -> Nullable<Binary>,
    }
}
