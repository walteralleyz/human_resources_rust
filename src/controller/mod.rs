pub mod funcionario_controller {
    use rocket::serde::json::{ Value, Json };
    use rocket::serde::json::serde_json::json;

    use crate::domain::funcionario::Funcionario;
    use crate::infra::funcionario_dao::FuncionarioDao;
    use crate::infra::pessoa_dao::PessoaDao;

    #[post("/", format = "json", data = "<funcionario>")]
    pub fn create_funcionario(funcionario: Json<Funcionario>) -> Value {
        let pessoa_model = funcionario.get_pessoa().to_model();
        let pessoa_id = PessoaDao::save(pessoa_model);

        todo!("TODO: Criar esse ID dinamicamente");
        let funcionario_model = funcionario.to_model(Some(1));
        let funcionario_id = FuncionarioDao::save(funcionario_model);

        let message = format!("Salvo com id: {}", funcionario_id);
        json!({ "success": message.as_str() })
    }

    #[get("/<id>")]
    pub fn get_by_id(id: i32) -> Value {
        let funcionario = FuncionarioDao::find_by_id(id);
        let funcionario = funcionario.unwrap();
        let pessoa = PessoaDao::find_by_id(funcionario.pessoa_id);
        let funcionario = funcionario.to_domain(pessoa.unwrap());

        json!({ "success": funcionario })
    }
}