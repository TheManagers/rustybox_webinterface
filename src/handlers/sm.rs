extern crate iron;

use iron::{Handler, IronResult, AfterMiddleware, Request, Response, Chain};

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
