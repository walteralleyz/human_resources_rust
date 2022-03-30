use rocket::serde::{ Serialize, Deserialize };
use crate::model::pessoas::Pessoas;
use crate::utils::date::Date;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Pessoa {
    id: Option<i32>,
    nome: String,
    data_de_nascimento: Date,
    email: String,
}

impl Pessoa {
    pub fn new(id: i32, nome: String, data_de_nascimento: Date, email: String) -> Self {
        Pessoa { id: Some(id), nome, data_de_nascimento, email }
    }

    pub fn to_model(&self) -> Pessoas {
        Pessoas {
            id: None,
            nome: self.nome.clone(),
            data_de_nascimento: self.data_de_nascimento.parse_string(),
            email: self.email.clone()
        }
    }
}
