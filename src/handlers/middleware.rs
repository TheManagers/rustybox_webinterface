use iron::{Handler, IronResult, AfterMiddleware, Request, Response, Chain};
use iron::error::{IronError};
use iron::status;
use router::{NoRoute};
use iron::modifiers::Redirect;
use iron_sessionstorage;
use iron_sessionstorage::SessionRequestExt;
use handlers::login;
use url::Url;

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

pub struct TargetUrl {
    pub url: String
}

impl iron_sessionstorage::Value for TargetUrl {
    fn get_key() -> &'static str { "url" }
    fn into_raw(self) -> String { self.url }
    fn from_raw(value: String) -> Option<Self> {
        if value.is_empty() {
            None
        } else {
            Some(TargetUrl { url: value })
        }
    }
}

impl AfterMiddleware for AuthorizationCheck {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        if try!(req.session().get::<login::Login>()).is_none() {
            let target: Url = req.url.clone().into();
            req.session().set(TargetUrl {url: String::from(target.as_str())}).expect("Expected: Session write");
            return Ok(Response::with((status::Found, Redirect(url_for!(req, "login")))));
        }
        Ok(res)
    }
}

pub struct SelectiveMiddleWare2 {
    chain: Chain,
}

impl SelectiveMiddleWare2 {
    /// Create a new SelectiveMiddleWare instance with the given BeforeMiddleware.
    pub fn new<H: Handler, M: AfterMiddleware>(handler: H, m: Vec<M>) -> Self {

        let mut chain = Chain::new(handler);
        for item in m.into_iter() {
            chain.link_after(item);
        }

        SelectiveMiddleWare2 {
            chain: chain
        }
    }
}

impl Handler for SelectiveMiddleWare2 {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        self.chain.handle(req)
    }
}