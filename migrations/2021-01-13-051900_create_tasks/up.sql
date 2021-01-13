-- Your SQL goes here
CREATE TABLE tasks (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users(id),
    content VARCHAR NOT NULL,
    finished BOOLEAN NOT NULL DEFAULT 'f',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

CREATE INDEX tasks_user_id ON tasks (user_id);