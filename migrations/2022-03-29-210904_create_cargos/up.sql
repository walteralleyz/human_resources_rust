-- Your SQL goes here
CREATE TABLE cargos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cargo CHAR(30) NOT NULL UNIQUE
);