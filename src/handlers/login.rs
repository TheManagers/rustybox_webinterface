use iron::prelude::*;
use iron::status;
use iron::modifiers::Redirect;
use iron::modifiers::RedirectRaw;
use reqwest::Method;
use iron_sessionstorage;
use iron_sessionstorage::traits::*;
use urlencoded::UrlEncodedBody;
use hbs::Template;
use hbs::handlebars::to_json;
use wither::Model;
use persistent::Read as PRead;
use mongodb::ThreadedClient;

use models;
use db;
use helper;

use handlers::middleware::TargetUrl;

pub struct Login {
    username: String
}

impl iron_sessionstorage::Value for Login {
    fn get_key() -> &'static str { "logged_in_user" }
    fn into_raw(self) -> String { self.username }
    fn from_raw(value: String) -> Option<Self> {
        if value.is_empty() {
            None
        } else {
            Some(Login { username: value })
        }
    }
}

pub fn login(req: &mut Request) -> IronResult<Response> {
    #[derive(Serialize, Debug)]
    struct Data {
        failed: bool,
        errmessage: String
    }
    let data = Data {
        failed: true,
        errmessage: String::from("")
    };
    if req.method == Method::Post {
        let conn = get_mongodb_connection!(req);
        let pusername = {
            let formdata = iexpect!(req.get_ref::<UrlEncodedBody>().ok());
            iexpect!(formdata.get("username"))[0].to_owned()
        };
        let password = {
            let formdata = iexpect!(req.get_ref::<UrlEncodedBody>().ok());
            iexpect!(formdata.get("password"))[0].to_owned()
        };
        match models::user::User::find_one(conn.db("rbox"),
            Some(doc!{"username": pusername, "password": helper::encrypt_password(password)}),
            None
        ).expect("Not successfull lookup") {
            None => {
                let data = Data {
                    failed: true,
                    errmessage: String::from("Benutzername oder Password falsch")
                };
                let mut resp = Response::new();
                resp.set_mut(Template::new("login", to_json(&data))).set_mut(status::Ok);
                return Ok(resp);
            },
            Some(user) => {
                try!(req.session().set(Login { username: user.username }));
                match try!(req.session().get::<TargetUrl>()) {
                    Some(target) => {
                        req.session().set(TargetUrl { url: String::from("") }).expect("Expect: Session write");
                        return Ok(Response::with((status::Found, RedirectRaw(target.url))));
                    },
                    None => return Ok(Response::with((status::Found, Redirect(url_for!(req, "index")))))
                }
            }
        }

    } else {
        // Am i already logged in? 
        if try!(req.session().get::<Login>()).is_some() {
            return Ok(Response::with((status::Found, Redirect(url_for!(req, "index")))));
        }
    }

    let mut resp = Response::new();
    resp.set_mut(Template::new("login", to_json(&data))).set_mut(status::Ok);
    Ok(resp)
}

pub fn logout(req: &mut Request) -> IronResult<Response> {
    try!(req.session().clear());
    Ok(Response::with((status::Found, Redirect(url_for!(req, "index")))))
}