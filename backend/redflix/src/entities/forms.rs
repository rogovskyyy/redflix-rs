use rocket::http::RawStr;

pub struct LoginForm<'a> {
    pub email: &'a RawStr,
    pub password: &'a RawStr
}

pub struct RegisterForm<'a> {
    pub email: &'a RawStr,
    pub password: &'a RawStr,
    pub password_confirm: &'a RawStr
}