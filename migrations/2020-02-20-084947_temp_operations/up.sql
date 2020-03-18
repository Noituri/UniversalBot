-- Your SQL goes here
CREATE TABLE temp_operations (
    id SERIAL PRIMARY KEY,
    server_id INT NOT NULL references servers(id),
    action_type INT NOT NULL,
    target_id VARCHAR NOT NULL,
    end_date TIMESTAMP NOT NULL,
    FOREIGN KEY (server_id) REFERENCES servers(id)
)
