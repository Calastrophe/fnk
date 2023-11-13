create table if not exists test (
        test_id uuid primary key default uuid_generate_v4(),
        teacher_id uuid not null,
        name text not null,
        closed bool not null default false,

        foreign key (teacher_id) references teacher(teacher_id)
);

create table if not exists result (
        id uuid primary key default uuid_generate_v4(),
        test_id uuid not null,
        name text not null,
        score int not null default 0,
        flagged bool not null default false,

        foreign key (test_id) references test(test_id)
);
