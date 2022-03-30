use crate::schema::*;
use crate::establish_connection;

#[derive(Insertable, Queryable, AsChangeset)]
#[table_name = "eventos_funcionario"]
pub struct EventosFuncionario {
    pub id: Option<i32>,
    funcionario_id: i32,
    evento_id: i32,
    data_evento: String
}