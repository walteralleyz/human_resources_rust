-- Your SQL goes here
CREATE TABLE pessoas (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nome CHAR(50) NOT NULL,
    data_de_nascimento CHAR(10) NOT NULL,
    email CHAR(50) NOT NULL UNIQUE
);