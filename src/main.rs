#![feature(plugin, decl_macro)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate rocket_cors;

use rocket_cors::{AllowedOrigins};
use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use routers::static_rocket_route_info_for_index;
use routers::static_rocket_route_info_for_new;
use routers::static_rocket_route_info_for_show;
use routers::static_rocket_route_info_for_delete;
use routers::static_rocket_route_info_for_update;


mod schema;
mod models;
mod db;
mod routers;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    
    let pool = db::init_pool(database_url);

    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:4200"]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allow_credentials: true,
        ..Default:: default()
    }
    .to_cors()
    .expect("Error creating cors fairing");
    rocket::ignite()
        .manage(pool)
        .mount("/", routes![index, new, show, update, delete],
    ).attach(cors)
}

fn main(){
    rocket().launch();
}

// fn main() {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    
//     let conn = PgConnection::establish(&database_url).unwrap();
    
//     let homework = models::NewHomework {
//         title: String::from("Kruger meeting"),
//         published: true,
//     };
    
//     if models::Homework::insert(homework, &conn){
//         println!("success");
//     }else{
//         println!("failed")
//     }
//     // let filter = warp::path!("hello" / String).map(|name| format!("hello {}", name));

//     // println!("Entro al servidor en el puerto 8080");
//     // warp::serve(filter).run(([0,0,0,0],8080)).await;
// }
