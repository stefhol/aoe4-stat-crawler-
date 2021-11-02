

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
), temp_user AS (
    INSERT INTO age_user(
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
INSERT INTO user_match_history(
    user_id,
    match_history_id
)
VALUES(
    (SELECT id from temp_user),
    (SELECT id from temp_match)
);

