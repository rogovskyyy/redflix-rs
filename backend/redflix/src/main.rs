#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate postgres_query_macro;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::{ StaticFiles };

mod plugins {
    pub mod session_handler;
    pub mod redis_db;
    pub mod postgresql_db;
}

mod entities {
    pub mod user;
    pub mod forms;
}

pub mod routes;

fn main() {
    
    rocket::ignite()
        // Includes render engine 
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![routes::browse::homepage])
        .mount("/", routes![routes::browse::browse])

        //.mount("/", routes![routes::browse::movie_id])

        //.mount("/", routes![routes::account::login])
        //.mount("/", routes![routes::account::logout])
        //.mount("/", routes![routes::account::new])

        //.mount("/", routes![routes::test_route::check])
        //.mount("/", routes![routes::test_route::set_name])
        //.mount("/", routes![routes::test_route::redis_get_session])
        //.mount("/", routes![routes::test_route::redis_set_session])
        .launch();
}