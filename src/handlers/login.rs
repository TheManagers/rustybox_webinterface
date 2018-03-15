use iron::prelude::*;
use iron::status;
use iron::modifiers::Redirect;
use iron_sessionstorage;
use iron_sessionstorage::traits::*;
use urlencoded::UrlEncodedBody;
use hbs::Template;
use wither::Model;
use persistent::Read as PRead;
use mongodb::ThreadedClient;

use models;
use db;
use helper;

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
    // Am i already logged in? 
    if try!(req.session().get::<Login>()).is_some() {
        // TODO: Redirect to last page user tried to access (Session-Var)
        return Ok(Response::with((status::Found, Redirect(url_for!(req, "index")))));
    }

    let mut resp = Response::new();
    resp.set_mut(Template::new("login", ()))
        .set_mut(status::Ok);
    Ok(resp)
}


pub fn login_post(req: &mut Request) -> IronResult<Response> {
    // TODO: Anhand Passwort suchen. User-Password noch hashen.
    let conn = get_mongodb_connection!(req);
    let pusername = {
        let formdata = iexpect!(req.get_ref::<UrlEncodedBody>().ok());
        iexpect!(formdata.get("username"))[0].to_owned()
    };
    let password = {
        let formdata = iexpect!(req.get_ref::<UrlEncodedBody>().ok());
        iexpect!(formdata.get("password"))[0].to_owned()
    };
    let user = models::user::User::find_one(conn.db("rbox"),
            Some(doc!{"username": pusername, "password": helper::encrypt_password(password)}),
            None
        ).expect("Not successfull lookup")
        .expect("Not values Found");

    try!(req.session().set(Login { username: user.username }));
    
    Ok(Response::with((status::Found, Redirect(url_for!(req, "index")))))
}

pub fn logout(req: &mut Request) -> IronResult<Response> {
    try!(req.session().clear());
    Ok(Response::with((status::Found, Redirect(url_for!(req, "index")))))
}