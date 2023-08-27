CREATE COLLATION case_insensitive (
    provider = icu,
    locale = 'und-u-ks-level2',
    deterministic = false
);

CREATE TABLE accounts (
    id  SERIAL  PRIMARY KEY,

    username  VARCHAR(20)   NOT NULL  COLLATE case_insensitive  UNIQUE,
    gjp2      TEXT          NOT NULL, -- argon2 hashed (rubrub uses bcrypt but oh well)
    email     VARCHAR(254)  NOT NULL,

    -- todo: swap to proper rank system
    is_admin  INTEGER  NOT NULL  DEFAULT 0,

    -- 0: disabled, 1: only for friends, 2: open to all
    messages_enabled         INTEGER  NOT NULL  DEFAULT 2,
    comments_enabled         INTEGER  NOT NULL  DEFAULT 0,
    -- 0: disabled, 1: enabled
    friend_requests_enabled  INTEGER  NOT NULL  DEFAULT 1, -- frs enabled

    youtube_url  VARCHAR(30),
    twitter_url  VARCHAR(20),
    twitch_url   VARCHAR(20),

    created_at   TIMESTAMP   NOT NULL  DEFAULT CURRENT_TIMESTAMP
);