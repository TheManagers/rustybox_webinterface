// WebApp Based on https://github.com/rusts/team
#[macro_use]
extern crate iron;
#[macro_use]
extern crate router;
extern crate handlebars_iron as hbs;
extern crate params;
extern crate staticfile;
extern crate mount;
extern crate persistent;
extern crate iron_sessionstorage;
extern crate urlencoded;

extern crate mongodb;
extern crate r2d2;
extern crate r2d2_mongodb;

extern crate serde;
extern crate serde_json;
extern crate envy;

#[macro_use]
extern crate serde_derive;
extern crate wither;

extern crate crypto;

extern crate slack_hook;
extern crate time;
extern crate chrono;
extern crate diff;
extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;
#[macro_use]
extern crate lazy_static;
extern crate bson;
#[macro_use]
extern crate log;
extern crate fern;
extern crate url;
extern crate oauth2;
extern crate reqwest;

use std::error::Error;
//use std::path::Path;

use iron::prelude::*;
use router::Router;
use hbs::{HandlebarsEngine, DirectorySource};
//use staticfile::Static;
use mount::Mount;

use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::SignedCookieBackend;
use iron::middleware::{AroundMiddleware, Handler};

#[macro_use]
mod db;
mod handlers;
mod models;
mod helper;
mod env;

struct LoggerHandler<H: Handler> {
    logger: Logger,
    handler: H,
}
impl<H: Handler> Handler for LoggerHandler<H> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let entry = time::precise_time_ns();
        let res = self.handler.handle(req);
        let time = time::precise_time_ns() - entry;
        self.logger.log(req, res.as_ref(), time);
        res
    }
}
struct Logger;
impl Logger {
    fn log(&self, req: &Request, res: Result<&Response, &IronError>, time: u64) {
        info!("Request: {:?}\nResponse: {:?}\nResponse-Time: {:?}",
              req,
              res,
              time)
    }
}
impl AroundMiddleware for Logger {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(LoggerHandler {
                     logger: self,
                     handler: handler,
                 }) as Box<Handler>
    }
}

fn setup_fern(level: log::LogLevelFilter, verbose: bool) {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!("[{}][{}/{}:{}][{}] {}",
                                    chrono::Local::now().to_rfc3339(),
                                    record.location().module_path(),
                                    record.location().file(),
                                    record.location().line(),
                                    record.level(),
                                    message))
        })
        .level(level)
        .chain(std::io::stdout())
        //.chain(fern::log_file("output.log"))
        .filter(move |meta: &log::LogMetadata| verbose || meta.target().starts_with("team"))
        .apply()
        .unwrap()
}

fn main() {
    setup_fern(log::LogLevelFilter::Error, false);

    let router = handlers::router::create_router();

    let mut mount = Mount::new();
    /*mount.mount("/css", Static::new(Path::new("./public/css/")));
    mount.mount("/js", Static::new(Path::new("./public/js/")));
    mount.mount("/img", Static::new(Path::new("./public/img/")));
    mount.mount("/fonts", Static::new(Path::new("./public/fonts/")));
    mount.mount("/codemirror", Static::new(Path::new("./public/codemirror/")));
    mount.mount("/favicons", Static::new(Path::new("./public/favicons/")));*/
    mount.mount("/", router);

    let mut chain = Chain::new(mount);

    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./templates/", ".hbs")));
    if let Err(r) = hbse.reload() {
        panic!("{}", r.description());
    }
    chain.link_after(hbse);
        match db::get_pool(&env::CONFIG.team_database_url.as_str(), env::CONFIG.team_database_port) {
        Ok(pool) => chain.link(persistent::Read::<db::Mongodb>::both(pool)),
        Err(err) => {
            error!("MongoDB: {}", err);
            std::process::exit(-1);
        }
    };

    let secret = b"FLEo9NCJDhZbBaT".to_vec();
    chain.link_around(SessionStorage::new(SignedCookieBackend::new(secret)));

    chain.around(Logger);
    chain.link_after(handlers::middleware::Custom404);

    let listen = format!("{}:{}", "0.0.0.0", &env::CONFIG.port);
    info!("Listen {:?}", listen);
    println!("running on localhost:3000");
    Iron::new(chain).http(listen).unwrap();
}