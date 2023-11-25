create table if not exists question (
        level int not null primary key,
        question text not null,
        image_path text
);

