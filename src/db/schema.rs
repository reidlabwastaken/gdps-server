// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        #[max_length = 20]
        username -> Varchar,
        gjp2 -> Text,
        #[max_length = 254]
        email -> Varchar,
        is_admin -> Int4,
        messages_enabled -> Int4,
        comments_enabled -> Int4,
        friend_requests_enabled -> Int4,
        #[max_length = 30]
        youtube_url -> Nullable<Varchar>,
        #[max_length = 20]
        twitter_url -> Nullable<Varchar>,
        #[max_length = 20]
        twitch_url -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}
