create table users (
    id          bigserial primary key not null,
    name        varchar(255) unique not null,
    email       varchar(320) unique not null,
    password    bytea not null,

    created_on  timestamptz not null,
    updated_on  timestamptz not null
);
