CREATE TYPE friends_ideas_type_enum AS ENUM ('gift', 'conversation', 'place', 'activity', 'other');

CREATE TABLE friends_ideas (
    id VARCHAR NOT NULL PRIMARY KEY,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    friend_id VARCHAR NOT NULL REFERENCES friends(id) ON DELETE CASCADE,
    idea_type friends_ideas_type_enum NOT NULL
);
