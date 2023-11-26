create table if not exists test (
        id uuid primary key default uuid_generate_v4(),
        teacher_id uuid not null,
        name text not null,
        closed bool not null default false,

        foreign key (teacher_id) references teacher(id)
);

create table if not exists result (
        id uuid primary key default uuid_generate_v4(),
        test_id uuid not null,
        name text not null,
        level int not null default 1,

        foreign key (test_id) references test(id)
);
