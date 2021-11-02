

with temp_match AS (
INSERT INTO match_history(
    elo,
    eloRating,
    rank,
    wins,
    losses,
    winStreak
)
VALUES(
    200,
    1,
    1,
    10,
    3,
    10
) RETURNING id
), temp_player AS (
    INSERT INTO player(
    rlUserId,
    userName,
    region,
    avatarUrl
)
VALUES(
    3,
    'TEST',
    '0',
    'http://'
) RETURNING id
)
INSERT INTO player_match_history(
    player_id,
    match_history_id
)
VALUES(
    (SELECT id from temp_player),
    (SELECT id from temp_match)
);

