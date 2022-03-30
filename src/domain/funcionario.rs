use rocket::serde::{ Serialize, Deserialize };

use crate::domain::cargo::Cargo;
use crate::domain::pessoa::Pessoa;
use crate::model::funcionarios::Funcionarios;
use crate::utils::date::Date;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Funcionario {
    id: Option<i32>,
    pessoa: Pessoa,
    salario: f64,
    cargo: Cargo,
    matricula: i32,
    ativo: bool,
}

impl Funcionario {
    pub fn new(id: i32, pessoa: Pessoa, salario: f64, cargo: Cargo, matricula: i32, ativo: bool) -> Self {
        Funcionario { id: Some(id), pessoa, salario, cargo, matricula, ativo }
    }

    pub fn to_model(&self, pessoa_id: Option<i32>) -> Funcionarios {
        Funcionarios {
            id: None,
            pessoa_id: pessoa_id.unwrap(),
            cargo_id: self.cargo.to_index() as i32,
            salario: self.salario,
            matricula: self.matricula,
            ativo: self.ativo
        }
    }

    pub fn get_pessoa(&self) -> &Pessoa {
        &self.pessoa
    }

    pub fn promote_to(&mut self, cargo: Cargo) -> Result<&str, &str> {
        let novo_cargo = self.cargo.promote_to(cargo);

        if &novo_cargo == &self.cargo {
            return Err("Cannot promote!");
        }

        self.cargo = novo_cargo;
        Ok("success!")
    }
}
