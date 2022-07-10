CREATE TABLE IF NOT EXISTS marble_game_status
(
    argent_user     UUID PRIMARY KEY
        REFERENCES argent_users
            ON DELETE CASCADE,
    highest_cleared INTEGER NOT NULL
);