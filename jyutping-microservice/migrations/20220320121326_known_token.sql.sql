drop table if exists known_token;
--NOTE This table will contains all InputToken
create table if not exists known_token(
    id int GENERATED ALWAYS AS IDENTITY,
    word text not null,
    --TODO Make this into enum
    pos text not null,
    --NOTE Input token will always accept string in the jyutping field. It will be a single item only
    jyutping text not null,
    primary key(id),
    unique(word, pos, jyutping)
);
