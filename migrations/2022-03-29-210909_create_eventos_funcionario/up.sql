-- Your SQL goes here
CREATE TABLE eventos_funcionario (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    funcionario_id INTEGER NOT NULL,
    evento_id INTEGER NOT NULL,
    data_evento CHAR(10) NOT NULL
);