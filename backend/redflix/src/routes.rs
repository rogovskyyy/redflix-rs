pub mod browse {
    use rocket::http::Cookies;
    use rocket::response::Redirect;
    use crate::plugins::session_handler:: { Session };
    use rocket_contrib::templates::Template;
    use std::collections::HashMap;

    #[get("/")]
    pub fn homepage(mut cookies: Cookies) -> Template {
        if Session::is_logged(cookies) == true  {
            Redirect::to("/browse");
        } 
        let context: HashMap<String, String> = HashMap::new();
        Template::render("landing_page", &context)
    }

    #[get("/browse")]
    pub fn browse(cookies: Cookies) -> Template {
        if Session::is_logged(cookies) == false {
            Redirect::to("/");
        }

        let context: HashMap<String, String> = HashMap::new();
        Template::render("browse", &context)
    }

    #[get("/watch/<movie_id>")]
    pub fn movie_id(movie_id: String) -> String { format!(" Your movie is {}", movie_id) }
}

pub mod account {
    use rocket::http::Cookies;
    use rocket::response::Redirect;
    use crate::plugins::session_handler:: { Session };
    use rocket_contrib::templates::Template;
    use std::collections::HashMap;

    #[get("/account/login")]
    pub fn login(cookie: Cookies) {
        if Session::is_logged(cookie) == true  {
            Redirect::to("/browse");
        } else {
            let context: HashMap<String, String> = HashMap::new();
            Template::render("landing_page.template", &context);
        }
    }

    #[get("/account/new")]
    pub fn new() -> &'static str { "New" }
    #[post("/account/logout")]
    pub fn logout() -> &'static str { "Logout" }
}

pub mod test {
    use rocket::http::{Cookie, Cookies};
    use rocket::response::Redirect;
    #[allow(unused_imports)]
    use crate::entities::user::{ User };

    #[get("/test/check")]
    pub fn check(cookie: Cookies) -> String {
        let val = cookie.get("name").map(|c| c.value());
        let printer = match val {
            Some(v) => String::from(v),
            None => String::new()
        };
        format!("Your name is {}", printer)
    }

    #[get("/test/set/<value>")]
    pub fn set_name(mut cookie: Cookies, value: String) -> Redirect {
        let private_cookie = Cookie::build("name", value)
            .path("/")
            .secure(true)
            .finish();

        cookie.add(private_cookie);
        Redirect::to("/test/check")
    }
}