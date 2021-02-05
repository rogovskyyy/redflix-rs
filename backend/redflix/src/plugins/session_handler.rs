extern crate redis;

#[allow(unused_imports)]
use crate::plugins::redis_db::{ RedisEngine };
#[allow(unused_imports)]
use rocket::http::{Cookie, Cookies};
use crate::plugins::postgresql_db::{ PostgreSQLEngine };
use rocket::request::Form;
use crate::entities::forms::{ LoginForm };

pub struct Session { }

#[allow(dead_code)]
impl Session {

    pub fn is_logged(cookies: Cookies) -> bool {
        match cookies.get("ssid").map(|c| c.value()) {
            Some(_) => true,
            None => false
        }
    }

    pub fn create_session(cookies: Cookies, input: Form<LoginForm>) {
        if Session::is_logged(cookies) == false {

            let result = PostgreSQLEngine::execute(
                "SELECT users.email FROM users WHERE users.username = $1 AND users.password = $2", 
                vec![input.email.to_string(), input.password.to_string()]
            );

        }
    }
}