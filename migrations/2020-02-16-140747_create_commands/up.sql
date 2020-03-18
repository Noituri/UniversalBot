-- Your SQL goes here
CREATE TABLE commands (
    id SERIAL PRIMARY KEY,
    server_id INT NOT NULL references servers(id),
    command_name VARCHAR NOT NULL,
    disabled_channels TEXT[] NOT NULL,
    FOREIGN KEY (server_id) REFERENCES servers(id)
)
