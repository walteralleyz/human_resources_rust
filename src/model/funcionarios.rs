use crate::domain::cargo::Cargo;
use crate::domain::funcionario::Funcionario;
use crate::domain::pessoa::Pessoa;
use crate::schema::*;
use crate::establish_connection;
use crate::model::pessoas::Pessoas;

#[derive(Insertable, Queryable, AsChangeset)]
#[table_name = "funcionarios"]
pub struct Funcionarios {
    pub id: Option<i32>,
    pub cargo_id: i32,
    pub pessoa_id: i32,
    pub matricula: i32,
    pub ativo: bool,
    pub salario: f64
}

impl Funcionarios {
    pub fn to_domain(&self, pessoa: Pessoas) -> Funcionario {
        Funcionario::new(
            self.id.unwrap(),
            pessoa.to_domain(),
            self.salario,
            Cargo::from_index(self.cargo_id),
            self.matricula,
            self.ativo
        )
    }
}
