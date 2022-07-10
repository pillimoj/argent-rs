CREATE TABLE IF NOT EXISTS checklists (
    id uuid PRIMARY KEY,
    "name" TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS checklistitems (
    id uuid PRIMARY KEY,
    title TEXT NOT NULL,
    done BOOLEAN NOT NULL,
    created_at TIMESTAMP NOT NULL,
    checklist uuid NOT NULL,
    CONSTRAINT fk_checklistitems_checklist_id
        FOREIGN KEY (checklist)
        REFERENCES checklists(id)
        ON DELETE RESTRICT
        ON UPDATE RESTRICT
);
