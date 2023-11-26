create table if not exists question (
        id uuid primary key default uuid_generate_v4(),
        level int not null,
        question text not null,
        image_path text
);

INSERT INTO question (level, question, image_path) VALUES
    (1, 'Draw a picture of yourself in the box.', NULL),
    (1, 'Draw a picture of an animal in the box.', NULL),
    (1, 'Draw a picture of a tree in the box.', NULL),
    (2, 'Write your name in the box.', NULL),
    (2, 'Write the name of someone you know.', NULL),
    (2, 'Write the name of your favorite character.', NULL),
    (3, 'Spell the word in the box.', 'cat.jpg'),
    (3, 'Spell the word in the box.', 'pig.jpg'),
    (3, 'Spell the word in the box.', 'hat.jpg'),
    (4, 'Spell the word in the box.', 'crab.jpg'),
    (4, 'Spell the word in the box.', 'tree.jpg'),
    (4, 'Spell the word in the box.', 'frog.jpg'),
    (5, 'Spell the word in the box.', 'cheetah.jpg'),
    (5, 'Spell the word in the box.', 'church.jpg'),
    (5, 'Spell the word in the box.', 'cruise.jpg'),
    (6, 'Spell the word in the box.', 'birthday.jpg'),
    (6, 'Spell the word in the box.', 'caterpillar.jpg'),
    (6, 'Spell the word in the box.', 'playground.jpg'),
    (7, 'Can you write one word of something you like to do in the box?', NULL),
    (7, 'Can you write one word of something you like to do in the box?', NULL),
    (7, 'Can you write one word of something you like to do in the box?', NULL),
    (8, 'Write a sentence about your family, if you can write one sentence, write more.', NULL),
    (8, 'Write a sentence about your favorite season, if you can write one sentence, write more.', NULL),
    (8, 'Write a sentence about your favorite holiday, if you can write one sentence, write more.', NULL)
