-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL
);

INSERT INTO users (username) VALUES ('admin');

CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL
);

INSERT INTO roles (name, description) VALUES ('admin', 'Admin users');
INSERT INTO roles (name, description) VALUES ('signer', 'Signers');

CREATE TABLE user_roles (
  id SERIAL PRIMARY KEY,
  user_id INTEGER REFERENCES users(id),
  role_id INTEGER REFERENCES roles(id)
);

INSERT INTO user_roles (user_id, role_id) VALUES (1, 1);