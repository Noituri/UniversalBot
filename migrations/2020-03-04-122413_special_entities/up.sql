-- Your SQL goes here
CREATE TABLE special_entities (
    id SERIAL PRIMARY KEY,
    server_id INT NOT NULL references servers(id),
    entity_type INT NOT NULL,
    entity_id VARCHAR NOT NULL,
    FOREIGN KEY (server_id) REFERENCES servers(id)
)