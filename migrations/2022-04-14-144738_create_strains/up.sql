-- Your SQL goes here
CREATE TYPE species AS ENUM('indica', 'sativa', 'hybrid');

CREATE TABLE strains (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    species SPECIES NOT NULL
);
