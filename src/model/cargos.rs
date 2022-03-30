use crate::schema::*;
use crate::establish_connection;

#[derive(Insertable, Queryable, AsChangeset)]
#[table_name = "cargos"]
pub struct Cargos {
    pub id: Option<i32>,
    cargo: String
}