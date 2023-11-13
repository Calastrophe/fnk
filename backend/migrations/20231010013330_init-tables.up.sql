
create table if not exists teacher (
        teacher_id uuid primary key default uuid_generate_v4(),
        username text not null,
        email text not null unique,
        password text not null
);
