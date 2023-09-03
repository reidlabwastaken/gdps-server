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
        created_at -> Text,
    }
}

diesel::table! {
    levels (id) {
        id -> Int4,
        created_at -> Text,
        modified_at -> Text,
        #[max_length = 20]
        name -> Varchar,
        user_id -> Int4,
        #[max_length = 140]
        description -> Varchar,
        original -> Nullable<Int4>,
        game_version -> Int4,
        binary_version -> Int4,
        password -> Nullable<Text>,
        requested_stars -> Nullable<Int4>,
        unlisted -> Int4,
        version -> Int4,
        extra_data -> Bytea,
        level_info -> Bytea,
        editor_time -> Int4,
        editor_time_copies -> Int4,
        song_id -> Int4,
        length -> Int4,
        objects -> Int4,
        coins -> Int4,
        has_ldm -> Int4,
        two_player -> Int4,
        downloads -> Int4,
        likes -> Int4,
        difficulty -> Nullable<Int4>,
        community_difficulty -> Nullable<Int4>,
        demon_difficulty -> Nullable<Int4>,
        stars -> Nullable<Int4>,
        featured -> Int4,
        epic -> Int4,
        rated_coins -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        udid -> Nullable<Text>,
        account_id -> Nullable<Int4>,
        registered -> Int4,
        username -> Text,
        stars -> Int4,
        demons -> Int4,
        coins -> Int4,
        user_coins -> Int4,
        diamonds -> Int4,
        orbs -> Int4,
        creator_points -> Int4,
        completed_levels -> Int4,
        icon_type -> Int4,
        color1 -> Int4,
        color2 -> Int4,
        cube -> Int4,
        ship -> Int4,
        ball -> Int4,
        ufo -> Int4,
        wave -> Int4,
        robot -> Int4,
        spider -> Int4,
        explosion -> Int4,
        special -> Int4,
        glow -> Int4,
        created_at -> Text,
        last_played -> Text,
        is_banned -> Int4,
        is_banned_upload -> Int4,
    }
}

diesel::joinable!(levels -> users (user_id));
diesel::joinable!(users -> accounts (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    levels,
    users,
);
