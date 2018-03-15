use iron::{Handler, IronResult, AfterMiddleware, Request, Response, Chain};
use iron::error::{IronError};
use iron::status;
use router::{NoRoute};
use iron::modifiers::Redirect;
use iron_sessionstorage::SessionRequestExt;
use handlers::login;

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