CREATE TABLE IF NOT EXISTS owner
(
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS pet
(
    id SERIAL PRIMARY KEY NOT NULL,
    owner_id INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    animal_type VARCHAR(255) NOT NULL,
    color VARCHAR(255),

    CONSTRAINT fk_pet_owner_id FOREIGN KEY (owner_id) REFERENCES pet(id)
);