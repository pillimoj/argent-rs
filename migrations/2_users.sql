CREATE TABLE IF NOT EXISTS argent_users
(
    id    UUID PRIMARY KEY,
    name  TEXT      NOT NULL,
    email TEXT      NOT NULL,
    role  TEXT      NOT NULL
);

CREATE TABLE IF NOT EXISTS checklist_access
(
    checklist   UUID NOT NULL,
    argent_user UUID NOT NULL,
    access_type TEXT NOT NULL,
    PRIMARY KEY (argent_user, checklist),
    CONSTRAINT fk_checklist_access_checklist_id
        FOREIGN KEY (checklist)
            REFERENCES checklists (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
    CONSTRAINT fk_checklist_access_argent_user_id
        FOREIGN KEY (argent_user)
            REFERENCES argent_users (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);
