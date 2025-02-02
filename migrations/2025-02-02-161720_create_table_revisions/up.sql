create table revisions (
    id bigserial primary key,
    parent_id bigint default null
        references revisions (id)
            on delete cascade,
    content_id bigint not null
        references contents (id)
            on delete restrict,
    
    page_id bigint not null,
    user_id bigint not null
        references users (id)
            on delete set default,
    created_on timestamptz not null
        default now()
);
