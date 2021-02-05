extern crate tokio_postgres as pgsql;

#[allow(dead_code)]
#[derive(FromSqlRow)]
pub struct User {
    id: i32,
    email: String,
    password: String,
    register_date: String,
    is_active: bool
}