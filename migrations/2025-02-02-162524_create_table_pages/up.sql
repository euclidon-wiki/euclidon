create table pages (
    id          bigserial primary key,
    title       varchar(255) not null unique,
    rev_id      bigint not null
        references revisions (id)
            on delete restrict,

    created_on timestamptz not null
);
