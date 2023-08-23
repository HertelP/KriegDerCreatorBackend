-- Add migration script here
CREATE TABLE users(
    id uuid NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    name TEXT NOT NULL,
    level INTEGER NOT NULL,
    follower INTEGER NOT NULL,
    progress INTEGER NOT NULL,
    token INTEGER NOT NULL,
    gold INTEGER NOT NULL,
    energy INTEGER NOT NULL,
    wood INTEGER NOT NULL,
    leather INTEGER NOT NULL,
    iron INTEGER NOT NULL,
    fabric INTEGER NOT NULL,
    characters uuid[],
    inventory uuid[],
    inventorynums INTEGER[]
);
