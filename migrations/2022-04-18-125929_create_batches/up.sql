-- Your SQL goes here
CREATE TABLE growers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE batches (
    id SERIAL PRIMARY KEY,
    strain_id INT NOT NULL,
    harvest_date DATE NULL,
    final_test_date DATE NULL,
    package_date DATE NULL,
    grower_id INT NOT NULL,
    thc_content FLOAT4 NOT NULL,
    cbd_content FLOAT4 NOT NULL,
    FOREIGN KEY (strain_id) REFERENCES strains (id) ON DELETE CASCADE,
    FOREIGN KEY (grower_id) REFERENCES growers (id) ON DELETE CASCADE
);
