use crate::schema::*;
use crate::establish_connection;

#[derive(Insertable, Queryable, AsChangeset)]
#[table_name = "eventos"]
pub struct Eventos {
    pub id: Option<i32>,
    pub evento: String
}