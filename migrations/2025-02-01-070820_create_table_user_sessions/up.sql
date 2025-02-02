create table user_sessions (
    token       varchar(24) primary key,
    user_id     bigint not null
        references users
            on delete cascade,

    expire_on   timestamptz default null
);
