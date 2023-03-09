CREATE TYPE friends_events_type_enum AS ENUM ('catchup', 'life_event', 'other');

CREATE TABLE friends_events (
    id VARCHAR NOT NULL PRIMARY KEY,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    friend_id VARCHAR NOT NULL REFERENCES friends(id) ON DELETE CASCADE,
    event_type friends_events_type_enum NOT NULL
);
