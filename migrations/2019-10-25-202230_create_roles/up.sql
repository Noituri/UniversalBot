-- Your SQL goes here
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    server_id INT NOT NULL references servers(id),
    role_id VARCHAR NOT NULL,
    perms TEXT[] NOT NULL DEFAULT '{}',
    FOREIGN KEY (server_id) REFERENCES servers(id)
)