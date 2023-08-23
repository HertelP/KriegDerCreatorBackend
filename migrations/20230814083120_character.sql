-- Add migration script here
CREATE TABLE characters(
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    name TEXT,
    level INTEGER,
    follower INTEGER,
    stars INTEGER,
    fragments INTEGER,
    -- Fragments werden für die Sterne benötigt 
    life INTEGER,
    attack INTEGER,
    armor INTEGER,
    initiative INTEGER,
    resistance INTEGER,
    dodge INTEGER,
    armorlevel INTEGER,
    weapon uuid REFERENCES equipment(id),
    trousers uuid REFERENCES equipment(id),
    helmet uuid REFERENCES equipment(id),
    body uuid REFERENCES equipment(id),
    belt uuid REFERENCES equipment(id),
    shoes uuid REFERENCES equipment(id)
);
