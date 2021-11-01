CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
create table user(
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    rlUserId int UNIQUE,
    userName varchar(255),
    region varchar(20),
    avatarUrl varchar(255)
);

create table match_history(
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    time DATETIME DEFAULT CURRENT_DATETIME(),
    elo int,
    eloRating int,
    rank int,
    region varchar(255),
    wins int,
    losses int,
    winStreak int
);

create table user_match_history(
    user_id uuid FOREIGN KEY REFERENCES user(id),
    match_history_id uuid FOREIGN KEY REFERENCES match_history(id)
);