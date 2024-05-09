-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id VARCHAR PRIMARY KEY, -- usr-UUID
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    user_role VARCHAR NOT NULL CHECK (user_role IN ('HOST', 'GUEST'))
);


CREATE TABLE offices (
    id VARCHAR PRIMARY KEY, -- ofc or spl
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    name VARCHAR NOT NULL,
    address_text VARCHAR NOT NULL,
    latitude FLOAT,
    longitude FLOAT,
    surface FLOAT NOT NULL,
    num_posts INT CHECK (num_posts >= 40 AND num_posts <= 180),
    price_per_post INT CHECK (price_per_post BETWEEN 300 AND 800),
    parent_office_id VARCHAR REFERENCES offices(id),
    owner_id VARCHAR NOT NULL REFERENCES users(id)
);


CREATE TABLE host_relations (
    user_id VARCHAR NOT NULL REFERENCES users(id),
    office_id VARCHAR NOT NULL REFERENCES offices(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, office_id, created_at)
);


CREATE TABLE guest_relations (
    user_id VARCHAR NOT NULL REFERENCES users(id),
    office_id VARCHAR NOT NULL REFERENCES offices(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, office_id, created_at)
);


CREATE TABLE contracts (
    id VARCHAR PRIMARY KEY, -- agr-UUID
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    host_id VARCHAR NOT NULL REFERENCES users(id),
    guest_id VARCHAR NOT NULL REFERENCES users(id),
    office_id VARCHAR NOT NULL REFERENCES offices(id),
    num_posts INT NOT NULL,
    monthly_amount INT NOT NULL,
    start_date TIMESTAMP WITH TIME ZONE NOT NULL,
    end_date TIMESTAMP WITH TIME ZONE NOT NULL,
    CHECK (end_date >= start_date + INTERVAL '4 months')
);
