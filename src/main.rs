#![feature(plugin, custom_derive, const_fn, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate diesel_codegen;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_derive;
extern crate serde_json;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema;
mod models;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();
    
    let homework = models::NewHomework {
        title: String::from("Kruger meeting"),
        published: true,
    };
    
    if models::Homework::insert(homework, &conn){
        println!("success");
    }else{
        println!("failed")
    }
    // let filter = warp::path!("hello" / String).map(|name| format!("hello {}", name));

    // println!("Entro al servidor en el puerto 8080");
    // warp::serve(filter).run(([0,0,0,0],8080)).await;
}
