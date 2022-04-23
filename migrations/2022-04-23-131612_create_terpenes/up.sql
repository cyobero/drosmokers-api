-- Your SQL goes here
CREATE TABLE terpenes (
    id SERIAL PRIMARY KEY,
    batch_id INT NOT NULL,
    caryophyllene FLOAT4 NULL,
    humulene FLOAT4 NULL,
    limonene FLOAT4 NULL,
    linalool FLOAT4 NULL,
    myrcene FLOAT4 NULL,
    pinene FLOAT4 NULL,
    FOREIGN KEY (batch_id) REFERENCES batches (id) ON DELETE CASCADE
);
