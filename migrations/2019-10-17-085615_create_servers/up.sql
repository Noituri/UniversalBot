-- Your SQL goes here
CREATE TABLE servers (
    id SERIAL PRIMARY KEY,
    guildid VARCHAR NOT NULL,
    prefix VARCHAR NOT NULL DEFAULT '.',
    enabledmodules TEXT[] NOT NULL DEFAULT '{}',
    disabledcommands TEXT[] NOT NULL DEFAULT '{}'
)