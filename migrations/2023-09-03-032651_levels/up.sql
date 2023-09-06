CREATE TABLE levels (
    id           SERIAL  PRIMARY KEY,
    created_at   TEXT    NOT NULL  DEFAULT (TO_CHAR(CURRENT_TIMESTAMP, 'YYYY-MM-DD HH24:MI:SS.MS')),
    modified_at  TEXT    NOT NULL  DEFAULT (TO_CHAR(CURRENT_TIMESTAMP, 'YYYY-MM-DD HH24:MI:SS.MS')),

    name         VARCHAR(20)   NOT NULL,
    user_id      INTEGER       NOT NULL  references users(id),
    description  VARCHAR(140)  NOT NULL  DEFAULT '',
    original     INTEGER,

    game_version    INTEGER  NOT NULL,
    binary_version  INTEGER  NOT NULL,

    password         TEXT,
    requested_stars  INTEGER,
    unlisted         INTEGER  NOT NULL  DEFAULT 0,

    version     INTEGER  NOT NULL  DEFAULT 0,
    extra_data  BYTEA     NOT NULL,
    level_info  BYTEA     NOT NULL,

    editor_time         INTEGER  NOT NULL,
    editor_time_copies  INTEGER  NOT NULL,

    song_id  INTEGER  NOT NULL,

    length      INTEGER           NOT NULL,
    length_real DOUBLE PRECISION  NOT NULL,
    objects     INTEGER           NOT NULL,
    coins       INTEGER           NOT NULL  DEFAULT 0,
    has_ldm     INTEGER           NOT NULL  DEFAULT 0,
    two_player  INTEGER           NOT NULL  DEFAULT 0,

    downloads             INTEGER  NOT NULL  DEFAULT 0,
    likes                 INTEGER  NOT NULL  DEFAULT 0,
    difficulty            INTEGER,
    community_difficulty  INTEGER,
    demon_difficulty      INTEGER,
    stars                 INTEGER,
    featured              INTEGER  NOT NULL  DEFAULT 0,
    epic                  INTEGER  NOT NULL  DEFAULT 0,
    rated_coins           INTEGER  NOT NULL  DEFAULT 0
);
