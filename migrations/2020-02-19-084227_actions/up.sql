-- Your SQL goes here
CREATE TABLE actions (
    id SERIAL PRIMARY KEY,
    server_id INT NOT NULL references servers(id),
    action_type INT NOT NULL,
    issuer VARCHAR NOT NULL,
    target VARCHAR,
    message VARCHAR NOT NULL,
    FOREIGN KEY (server_id) REFERENCES servers(id)
)