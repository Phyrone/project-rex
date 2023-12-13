DROP TABLE IF EXISTS user_publications_preference_interaction;
DROP TYPE IF EXISTS user_preference;
DROP TABLE IF EXISTS user_publication_reaction_interaction;
DROP TYPE IF EXISTS reaction_type;
DROP TABLE IF EXISTS attachment;
DROP TABLE IF EXISTS comment;

DROP TRIGGER IF EXISTS trg_message_notify ON message;
DROP FUNCTION IF EXISTS notify_channel_message;
DROP TABLE IF EXISTS message;

DROP TABLE IF EXISTS post;

DROP TABLE IF EXISTS guilded_channel;
DROP TYPE IF EXISTS channel_type;
DROP TABLE IF EXISTS guild;

DROP TABLE IF EXISTS channel_member;
DROP TABLE IF EXISTS channel;

ALTER TABLE IF EXISTS asset
    DROP COLUMN IF EXISTS
        first_by;
DROP TABLE IF EXISTS profile;
DROP TABLE IF EXISTS login_attempt;
DROP TABLE IF EXISTS account;
DROP TABLE IF EXISTS asset;
DROP TABLE IF EXISTS node;

-- drop types
DROP DOMAIN IF EXISTS email;
DROP DOMAIN IF EXISTS pub_name;
DROP DOMAIN IF EXISTS crypt_key;
DROP DOMAIN IF EXISTS rich_text;
DROP EXTENSION IF EXISTS citext;