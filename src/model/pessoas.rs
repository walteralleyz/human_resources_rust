use crate::domain::pessoa::Pessoa;
use crate::schema::*;
use crate::establish_connection;
use crate::utils::date::Date;

#[derive(Insertable, Queryable, AsChangeset)]
#[table_name = "pessoas"]
pub struct Pessoas {
    pub id: Option<i32>,
    pub nome: String,
    pub data_de_nascimento: String,
    pub email: String
}

impl Pessoas {
    pub fn to_domain(&self) -> Pessoa {
        Pessoa::new(
            self.id.unwrap(),
            self.nome.clone(),
            Date::from_string(self.data_de_nascimento.clone()),
            self.email.clone()
        )
    }
}