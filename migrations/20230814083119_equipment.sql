-- Add migration script here
CREATE TABLE equipment(
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    name TEXT,
    attack INTEGER
);
