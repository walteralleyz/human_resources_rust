#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

extern crate core;

mod test;

mod controller;
mod infra;
mod domain;
mod model;
mod utils;
mod schema;

use diesel::prelude::*;
use crate::controller::funcionario_controller::{ create_funcionario, get_by_id };

#[rocket::main]
async fn main() {
    let routes = routes![create_funcionario, get_by_id];

    match rocket::build().mount("/", routes).launch().await {
        Ok(_) => println!("Web App started at: localhost:8000"),
        Err(e) => panic!("Some error launching web server: {}", e.to_string())
    };
}

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("memory.sqlite").unwrap_or_else(
        |_| panic!("Error trying to connect!"))
}
