CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
create table age_user (
    id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY,
    rlUserId int UNIQUE,
    userName varchar(255),
    region varchar(20),
    avatarUrl varchar(255)
);

create table match_history(
    id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY,
    time TIMESTAMP DEFAULT NOW(),
    elo int,
    eloRating int,
    rank int,
    wins int,
    losses int,
    winStreak int
);

create table user_match_history(
    user_id uuid REFERENCES age_user(id),
    match_history_id uuid REFERENCES match_history(id)
);