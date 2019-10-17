-- Your SQL goes here
CREATE TABLE servers (
    id SERIAL PRIMARY KEY,
    guildID VARCHAR NOT NULL,
    ownerID VARCHAR NOT NULL,
    enabledModules TEXT[],
    enabledCommands TEXT[]
)