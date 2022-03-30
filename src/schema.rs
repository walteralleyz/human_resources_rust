table! {
    cargos (id) {
        id -> Nullable<Integer>,
        cargo -> Text,
    }
}

table! {
    eventos (id) {
        id -> Nullable<Integer>,
        evento -> Text,
    }
}

table! {
    eventos_funcionario (id) {
        id -> Nullable<Integer>,
        funcionario_id -> Integer,
        evento_id -> Integer,
        data_evento -> Text,
    }
}

table! {
    funcionarios (id) {
        id -> Nullable<Integer>,
        cargo_id -> Integer,
        pessoa_id -> Integer,
        matricula -> Integer,
        ativo -> Bool,
        salario -> Double,
    }
}

table! {
    pessoas (id) {
        id -> Nullable<Integer>,
        nome -> Text,
        data_de_nascimento -> Text,
        email -> Text,
    }
}

joinable!(funcionarios -> cargos (cargo_id));
joinable!(funcionarios -> pessoas (pessoa_id));

allow_tables_to_appear_in_same_query!(
    cargos,
    eventos,
    eventos_funcionario,
    funcionarios,
    pessoas,
);
