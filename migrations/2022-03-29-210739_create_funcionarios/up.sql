-- Your SQL goes here
CREATE TABLE funcionarios (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cargo_id INTEGER NOT NULL,
    pessoa_id INTEGER NOT NULL,
    matricula INTEGER NOT NULL UNIQUE,
    ativo BOOLEAN NOT NULL DEFAULT 1,
    CONSTRAINT fk_cargo
    FOREIGN KEY (cargo_id) REFERENCES cargos (id),
    CONSTRAINT fk_pessoa
    FOREIGN KEY (pessoa_id) REFERENCES pessoas (id)
);