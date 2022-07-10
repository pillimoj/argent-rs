CREATE TABLE IF NOT EXISTS wishlist_items
(
    id          UUID PRIMARY KEY,
    title       TEXT NOT NULL,
    description TEXT NOT NULL,
    taken_by    UUID
        REFERENCES argent_users
        ON DELETE SET NULL,
    argent_user UUID NOT NULL
        REFERENCES argent_users
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS wishlist_access
(
    wishlist_user   UUID NOT NULL
        REFERENCES argent_users
        ON DELETE CASCADE,
    access_user UUID NOT NULL
        REFERENCES argent_users
        ON DELETE CASCADE,
    PRIMARY KEY (wishlist_user, access_user)
);