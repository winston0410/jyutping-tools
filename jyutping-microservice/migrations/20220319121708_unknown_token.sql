drop table if exists unknown_token;
create table if not exists unknown_token(
    id int GENERATED ALWAYS AS IDENTITY,
    -- key contains the first longest unknown in sentence, unique
    key text not null,
    -- the original sentence with unknown input
    sentence text not null,
    primary key(id),
    unique (key)
);
