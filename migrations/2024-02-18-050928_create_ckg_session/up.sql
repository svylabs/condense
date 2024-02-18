-- Your SQL goes here
--CREATE type ckg_session_state as enum ('Requested', 'Initiated',  'Round1', 'Round2', 'Round3', 'Success', 'Error', 'Timedout');

--CREATE type ckg_session_participant_state as enum ('Initiated', 'Round1', 'Round2', 'Round3', 'Completed');

CREATE TABLE ckg_sessions (
  id SERIAL PRIMARY KEY,
  initiated_by INTEGER REFERENCES users(id) NOT NULL,
  threshold INTEGER NOT NULL,
  total_participants INTEGER NOT NULL,
  current_state VARCHAR NOT NULL DEFAULT 'Requested',
  ckg_session_timeout INTEGER NOT NULL,
  generated_public_key VARCHAR NOT NULL,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE ckg_session_participant_values (
  id SERIAL PRIMARY KEY,
  ckg_session_id INTEGER REFERENCES ckg_sessions(id) NOT NULL,
  participant_id INTEGER REFERENCES users(id) NOT NULL,
  current_state VARCHAR NULL,
  session_public_keys bytea NULL,
  round1_data bytea NULL,
  round2_data bytea NULL,
  round3_data bytea NULL,
  updated_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

