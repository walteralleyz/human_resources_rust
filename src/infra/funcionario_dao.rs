use diesel::prelude::*;
use crate::establish_connection;
use crate::model::funcionarios::Funcionarios;
use crate::schema::*;

pub struct FuncionarioDao;

impl FuncionarioDao {
    pub fn save(funcionario: Funcionarios) -> bool {
        diesel::insert_into(funcionarios::table)
            .values(funcionario)
            .execute(&establish_connection())
            .unwrap() > 0
    }

    pub fn find_by_id(id: i32) -> Option<Funcionarios> {
        match funcionarios::table.filter(funcionarios::id.eq(id))
            .first(&establish_connection()) {
            Ok(funcionario) => Some(funcionario),
            Err(_) => None
        }
    }
}