CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE match_type AS ENUM ('unranked', 'ranked', 'custom');
CREATE TYPE team_size AS ENUM ('1v1', '2v2', '3v3','4v4','custom');
CREATE TYPE versus AS ENUM ('ai', 'players');


create table player (
    id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY,
    rl_user_id bigint UNIQUE NOT NULL,
    username varchar(255) NOT NULL,
    region varchar(20) NOT NULL,
    avatar_url varchar(500)
);

create table match_history(
    id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY,
    time TIMESTAMP DEFAULT NOW(),
    elo int NOT NULL,
    elo_rating int NOT NULL,
    rank int NOT NULL,
    wins int NOT NULL,
    losses int NOT NULL,
    win_streak int NOT NULL,
    match_type match_type NOT NULL,
    versus versus NOT NULL,
    team_size team_size NOT NULL
);

create table player_match_history(
    player_id uuid REFERENCES player(id),
    match_history_id uuid REFERENCES match_history(id)
);