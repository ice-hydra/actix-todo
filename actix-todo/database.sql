DROP TABLE IF EXISTS todo_item;
DROP TABLE IF EXISTS todo_list;


CREATE TABLE todo_list(
    id SERIAL PRIMARY KEY,
    title VARCHAR(150) NOT NULL
);

CREATE TABLE todo_item(
    id SERIAL PRIMARY KEY,
    title VARCHAR(150) NOT NULL,
    checked BOOLEAN NOT NULL DEFAULT FALSE,
    list_id INTEGER NOT NULL,
    FOREIGN KEY(list_id) REFERENCES todo_list(id)
);

INSERT INTO todo_list (title) VALUES ('清单1'), ('清单2');

INSERT INTO todo_item (title, list_id) VALUES 
    ('ITEM 1', 1),
    ('ITEM 2', 1),
    ('ITEM A', 2);
