CREATE EXTENSION IF NOT EXISTS citext;
CREATE DOMAIN email AS citext
    CHECK ( value ~
            '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$' );
-- Alias for a domain name (pub_name = public name)
CREATE DOMAIN pub_name AS citext;
-- Alias for ed25519 public or private key
CREATE DOMAIN crypt_key AS BYTEA NOT NULL
    CHECK ( LENGTH(value) = 32 );

CREATE DOMAIN rich_text AS JSONB
    CHECK ( value IS NULL OR jsonb_typeof(value) = 'array' )
    DEFAULT '[]'::jsonb;


CREATE TABLE IF NOT EXISTS asset
(
    checksum   BYTEA     NOT NULL PRIMARY KEY
        CHECK ( LENGTH(checksum) = 32 ),
    first_seen TIMESTAMP NOT NULL DEFAULT NOW(),
    mime_type  VARCHAR   NULL     DEFAULT NULL,
    parent     BYTEA     NULL     DEFAULT NULL
        REFERENCES asset (checksum) ON UPDATE CASCADE ON DELETE CASCADE,
    metadata   JSONB     NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE node
(
    id      BIGSERIAL PRIMARY KEY,
    domains pub_name NOT NULL UNIQUE,
    -- notes from the admit to the corresponding node
    note    text       NOT NULL
);

-- Account,Profile etc.
CREATE TABLE account
(
    id             BIGSERIAL PRIMARY KEY,
    email          EMAIL   NOT NULL UNIQUE,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    active         BOOLEAN NOT NULL DEFAULT TRUE,
    session_secret BYTEA   NULL,
    admin          BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE login_attempt
(
    id         BIGSERIAL PRIMARY KEY,
    ip_address INET      NULL,
    timestamp  TIMESTAMP NOT NULL DEFAULT NOW(),
    account    BIGINT    NULL REFERENCES account (id)
        ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE profile
(
    id            BIGSERIAL PRIMARY KEY,
    account       BIGINT  NULL
        REFERENCES account (id) ON UPDATE CASCADE ON DELETE SET NULL,
    name          VARCHAR NOT NULL
        CHECK ( LENGTH(name) > 0 ),
    discriminator INTEGER NULL DEFAULT NULL,
    node          BIGINT  NULL
        REFERENCES node (id) ON UPDATE CASCADE ON DELETE RESTRICT,
    foreign_id    BIGINT  NULL,
    nickname      VARCHAR NULL,
    bio           rich_text    DEFAULT '[]'::rich_text,
    picture       BYTEA   NULL
        REFERENCES asset (checksum) ON UPDATE CASCADE ON DELETE SET NULL,

    -- check that either account or node is set
    CHECK ( account IS NOT NULL OR node IS NOT NULL),
    CHECK ( ((account IS NOT NULL)::integer + (node IS NOT NULL)::integer) = 1),
    UNIQUE NULLS NOT DISTINCT (name, discriminator, node)

);
-- TODO trigger which prevents using non picture assets as profile picture

-- Add first by to asset now since we need to create the profile table first
ALTER TABLE asset
    ADD COLUMN first_by BIGINT NULL
        REFERENCES profile (id) ON UPDATE CASCADE ON DELETE SET NULL;


CREATE TABLE channel
(
    id       BIGSERIAL PRIMARY KEY,
    name     VARCHAR NOT NULL UNIQUE,
    settings JSONB   NOT NULL DEFAULT '{}'::jsonb
);



CREATE TABLE channel_member
(
    channel       BIGINT    NOT NULL REFERENCES channel (id)
        ON UPDATE CASCADE ON DELETE CASCADE,
    profile       BIGINT    NOT NULL REFERENCES profile (id)
        ON UPDATE CASCADE ON DELETE CASCADE,
    since         TIMESTAMP NOT NULL DEFAULT NOW(),
    channel_admin BOOLEAN   NOT NULL DEFAULT FALSE,
    PRIMARY KEY (channel, profile)
);

CREATE TABLE message
(
    id         BIGSERIAL PRIMARY KEY,
    author     BIGINT    NOT NULL REFERENCES profile (id)
        ON UPDATE CASCADE ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    edited_at  TIMESTAMP NULL     DEFAULT NULL,
    channel    BIGINT    NOT NULL
        REFERENCES channel (id) ON UPDATE CASCADE ON DELETE CASCADE,
    pinned     BOOLEAN   NOT NULL DEFAULT FALSE,
    -- TODO tags
    -- TODO flags
    payload    rich_text NOT NULL
);

CREATE FUNCTION notify_channel_message()
    RETURNS TRIGGER AS
$$
BEGIN
    PERFORM pg_notify('channel/message', NEW.channel::text);
    RETURN NEW;
END
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_message_notify
    AFTER INSERT OR UPDATE
    ON message
    FOR EACH ROW
EXECUTE PROCEDURE notify_channel_message();

-- Guilds

CREATE TABLE IF NOT EXISTS guild
(
    id    BIGSERIAL PRIMARY KEY,
    name  VARCHAR NOT NULL,
    owner BIGINT  NULL REFERENCES profile (id)
        ON UPDATE CASCADE ON DELETE SET NULL

);

CREATE TYPE channel_type AS ENUM ('dummy','text', 'voice','announcement','stage','feed','forum');

CREATE TABLE guilded_channel
(
    channel  BIGSERIAL    NOT NULL PRIMARY KEY
        REFERENCES channel (id) ON UPDATE CASCADE ON DELETE CASCADE,
    guild    BIGINT       NOT NULL REFERENCES guild (id)
        ON UPDATE CASCADE ON DELETE CASCADE,
    parent   BIGINT       NULL REFERENCES channel (id)
        ON UPDATE CASCADE ON DELETE SET NULL,
    position INTEGER      NOT NULL DEFAULT 0,
    type     channel_type NOT NULL DEFAULT 'dummy'::channel_type
);


-- Posts
CREATE TABLE post
(
    id         BIGSERIAL PRIMARY KEY,
    author     BIGINT    NOT NULL REFERENCES profile (id)
        ON UPDATE CASCADE ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    edited_at  TIMESTAMP NULL     DEFAULT NULL,
    -- TODO tags
    -- TODO flags
    title      VARCHAR   NOT NULL,
    channel    BIGINT    NULL
        REFERENCES channel (id) ON UPDATE CASCADE ON DELETE CASCADE
);


CREATE TABLE comment
(
    id         BIGSERIAL PRIMARY KEY,

    author     BIGINT    NOT NULL REFERENCES profile (id)
        ON UPDATE CASCADE ON DELETE CASCADE,
    -- response to a comment or a post (like reddit, or youtube)
    parent     BIGINT    NULL
        REFERENCES comment (id) ON UPDATE CASCADE ON DELETE CASCADE,
    -- a post or another comment
    post       BIGINT    NOT NULL
        REFERENCES post (id) ON UPDATE CASCADE ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    edited_at  TIMESTAMP NULL     DEFAULT NULL
);


-- Additions and Interactions to posts, comments and messages (attachments, reactions, mentions etc.)
CREATE TABLE attachment
(
    -- One of these must be set
    post     BIGINT  NULL
        REFERENCES post (id) ON UPDATE CASCADE ON DELETE CASCADE,
    message  BIGINT  NULL
        REFERENCES message (id) ON UPDATE CASCADE ON DELETE CASCADE,
    comment  BIGINT  NULL
        REFERENCES comment (id) ON UPDATE CASCADE ON DELETE CASCADE,
    -- End oneof
    asset    BYTEA   NOT NULL
        REFERENCES asset (checksum) ON UPDATE CASCADE ON DELETE CASCADE,
    position INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (post, message, comment, asset),
    CHECK ( (post IS NOT NULL)::integer + (message IS NOT NULL)::integer + (comment IS NOT NULL)::integer = 1 )
);

CREATE TABLE user_publication_reaction_interaction
(
    id        BIGSERIAL PRIMARY KEY,
    -- One of these must be set
    post      BIGINT    NULL
        REFERENCES post (id) ON UPDATE CASCADE ON DELETE CASCADE,
    message   BIGINT    NULL
        REFERENCES message (id) ON UPDATE CASCADE ON DELETE CASCADE,
    comment   BIGINT    NULL
        REFERENCES comment (id) ON UPDATE CASCADE ON DELETE CASCADE,
    -- End oneof
    profile   BIGINT    NOT NULL
        REFERENCES profile (id) ON UPDATE CASCADE ON DELETE CASCADE,
    -- TODO table types of reactions f.e. emojies and custom emojies
    type      BIGINT    NOT NULL,
    placed_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE NULLS NOT DISTINCT (post, message, profile, type),
    CHECK ( (post IS NOT NULL)::integer + (message IS NOT NULL)::integer + (comment IS NOT NULL)::integer = 1 )
);

CREATE TYPE user_preference AS ENUM ('like','dislike');

CREATE TABLE user_publications_preference_interaction
(
    id      BIGSERIAL PRIMARY KEY,
    profile BIGINT          NOT NULL
        REFERENCES profile (id) ON UPDATE CASCADE ON DELETE CASCADE,
    -- One of these must be set
    post    BIGINT          NULL
        REFERENCES post (id) ON UPDATE CASCADE ON DELETE CASCADE,
    comment BIGINT          NULL
        REFERENCES comment (id) ON UPDATE CASCADE ON DELETE CASCADE,
    -- End oneof
    type    user_preference NOT NULL,
    UNIQUE NULLS NOT DISTINCT (profile, post, comment),
    CHECK ( (post IS NOT NULL)::integer + (comment IS NOT NULL)::integer = 1 )
)

-- TODO mentions