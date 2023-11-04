create table if not exists test (
        test_id uuid primary key default uuid_generate_v4(),
        teacher_id uuid not null,
        name text not null,
        closed bool not null,

        foreign key (teacher_id) references teacher(teacher_id)
);

create table if not exists result (
        id uuid primary key default uuid_generate_v4(),
        test_id uuid not null,
        name text not null,
        score int not null,
        finished bool not null,
        flagged bool not null,

        foreign key (test_id) references test(test_id)
);
