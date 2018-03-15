use iron::{Iron, Request, Response, IronResult, BeforeMiddleware, AfterMiddleware, Chain};
use iron::error::{IronError};
use iron::status;
use router::{Router, NoRoute};
use iron::modifiers::Redirect;
use iron_sessionstorage::SessionRequestExt;
use url::Url;
use iron::Url as IronUrl;
use handlers::login;
use helper;

pub struct Custom404;
#[derive(Clone, Debug)]
pub struct AuthorizationCheck;

impl AfterMiddleware for Custom404 {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        if err.error.is::<NoRoute>() {
            Ok(Response::with((status::NotFound, "Custom 404 response")))
        } else {
            Err(err)
        }
    }
}

impl AfterMiddleware for AuthorizationCheck {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        if try!(req.session().get::<login::Login>()).is_none() {
            return Ok(Response::with((status::Found, Redirect(url_for!(req, "login")))));
        }
        Ok(res)
    }
}