use diesel::prelude::*;
use crate::establish_connection;
use crate::model::pessoas::Pessoas;

use crate::schema::*;

pub struct PessoaDao;

impl PessoaDao {
    pub fn save(pessoa: Pessoas) -> bool {
        diesel::insert_into(pessoas::table)
            .values(pessoa)
            .execute(&establish_connection())
            .unwrap() > 0
    }

    pub fn find_by_id(id: i32) -> Option<Pessoas> {
        match pessoas::table.filter(pessoas::id.eq(id))
            .first(&establish_connection()) {
            Ok(pessoa) => Some(pessoa),
            Err(_) => None
        }
    }
}