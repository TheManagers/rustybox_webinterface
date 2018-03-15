use iron::prelude::*;
use iron::status;
use params::{Params, Value};

use mongodb::ThreadedClient;
use hbs::Template;
use hbs::handlebars::to_json;

use persistent::Read as PRead;
use db;
use models;
use helper;
use wither::Model;
use std::env;

const PAGINATES_PER: i32 = 10;

pub fn index_handler(_req: &mut Request) -> IronResult<Response> {
    #[derive(Serialize, Debug)]
    struct Data {
        //success: i32,
        logged_in: bool,
        //login_user: models::user::User,
        //feeds: Vec<models::post::Feed>,
        current_page: i32,
        total_page: i32,
        next_page: i32,
        prev_page: i32,
    }
    let data = Data {
        //success: result,
        logged_in: true,
        //login_user: user,
        current_page: 1,
        total_page: PAGINATES_PER,
        next_page: 2,
        prev_page: 0,
    };
    let mut resp = Response::new();
    resp.set_mut(Template::new("index", to_json(&data)))
        .set_mut(status::Ok);
    return Ok(resp);
}

pub fn setup_handler(req: &mut Request) -> IronResult<Response> {
    let conn = get_mongodb_connection!(req);

    let mut user = models::user::User {
        id: None,
        username: String::from("John"),
        password: helper::encrypt_password("mysecret".to_string()),
        hash: Some("theHASH".to_string())
    };


    let mut device = models::device::Device {
        id: None,
        appname: String::from("opus21"),
        name: String::from("meinDevice"),
        description: String::from("Hallo ich bin die Beschreibung"),
        active: true
    };

    // oder conn.clone().db("rbox") ??
    device.save(conn.db("rbox"), None).expect("Expected a successful save operation.");  // Insert into a MongoDB collection
    user.save(conn.db("rbox"), None).expect("Expected a successful save operation.");  // Insert into a MongoDB collection

    let mut resp = Response::new();
    resp.set_mut(Template::new("setup", ()))
        .set_mut(status::Ok);
    return Ok(resp);
}

pub fn newapp_handler(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("newapp", ()))
        .set_mut(status::Ok);
    return Ok(resp);
}

pub fn upload_handler(req: &mut Request) -> IronResult<Response> {
    //let params = req.get_ref::<Params>().unwrap();
    println!("{:?}", req.get_ref::<Params>());
    match req.get_ref::<Params>().unwrap().find(&["file"]) {
        Some(&Value::File(ref file)) => {
            use std::io::Read;
            use std::io::Write;
            use std::fs::File;
            use std::fs::create_dir;
            use std::path::PathBuf;
            // rename funkioniert nicht: { repr: Os { code: 18, message: "Invalid cross-device link" } }
            //fs::rename(&file.path, "/home/jonas/testfile").expect("Geht nicht");
            //println!("{:?}", file.path);
            let mut myfile = file.open().expect("Panik");
            let mut buffer = Vec::new();
            myfile.read_to_end(&mut buffer).expect("File panik");
            

            let bin = env::current_exe().expect("exe path");
            let mut target_dir = PathBuf::from(bin.parent().expect("bin parent"));
            target_dir.push("apps");
            if target_dir.is_dir() == false {
                create_dir(&target_dir).expect("Could not create dir");
            }
            target_dir.push("myapp.txt");
            let mut newfile = File::create(&target_dir).expect("File no save");
            newfile.write_all(&buffer).expect("no save");
        }
        _ => {
            println!("no file");
        }
    }
    let mut resp = Response::new();
    resp.set_mut(Template::new("upload", ()))
        .set_mut(status::Ok);
    return Ok(resp);
}



/*
pub fn index_handler(req: &mut Request) -> IronResult<Response> {
    let conn = get_pg_connection!(req);
    let mut login_user: models::user::User = models::user::User{..Default::default()};
    match handlers::account::current_user(req, &conn) {
        Ok(user) => { login_user = user; }
        Err(e) => { error!("Errored: {:?}", e); }
    }
    let login_id = login_user.id;
    if login_id == 0 {
        return Ok(Response::with((status::Found, Redirect(helper::redirect_url("/signin")))));
    }

    let page_param: String;

    {
        use params::{Params, Value};
        let map = req.get_ref::<Params>().unwrap();
        match map.get("page") {
            Some(&Value::String(ref name)) => {
                page_param = name.to_string();
            }
            _ => page_param = "1".to_string(),
        }
    }

    let mut resp = Response::new();

    #[derive(Serialize, Debug)]
    struct Data {
        logged_in: bool,
        login_user: models::user::User,
        feeds: Vec<models::post::Feed>,
        current_page: i32,
        total_page: i32,
        next_page: i32,
        prev_page: i32,
    }

    let mut page = page_param.parse::<i32>().unwrap();
    let offset = (page - 1) * PAGINATES_PER;
    let limit = PAGINATES_PER;

    let feeds: Vec<models::post::Feed>;
    let count: i32;

    match models::post::get_feeds(&conn, &offset, &limit) {
        Ok(feeds_db) => {
            feeds = feeds_db;
        }
        Err(e) => {
            error!("Errored: {:?}", e);
            return Ok(Response::with((status::InternalServerError)));
        }
    }

    match models::post::get_feed_count(&conn) {
        Ok(count_db) => {
            count = count_db;
        }
        Err(e) => {
            error!("Errored: {:?}", e);
            return Ok(Response::with((status::InternalServerError)));
        }
    }

    if page == 0 {
        page = 1;
    }
    let data = Data {
        logged_in: login_id != 0,
        login_user: login_user,
        feeds: feeds,
        current_page: page,
        total_page: count / PAGINATES_PER + 1,
        next_page: page + 1,
        prev_page: page - 1,
    };

    resp.set_mut(Template::new("index", to_json(&data)))
        .set_mut(status::Ok);
    return Ok(resp);
}*/
/*
pub fn search_handler(req: &mut Request) -> IronResult<Response> {
    let conn = get_pg_connection!(req);
    let mut login_user: models::user::User = models::user::User{..Default::default()};
    match handlers::account::current_user(req, &conn) {
        Ok(user) => { login_user = user; }
        Err(e) => { error!("Errored: {:?}", e); }
    }
    let login_id = login_user.id;
    if login_id == 0 {
        return Ok(Response::with((status::Found, Redirect(helper::redirect_url("/signin")))));
    }

    let keyword_param: String;
    let kind_param: String;
    let page_param: String;

    {
        use params::{Params, Value};
        let map = req.get_ref::<Params>().unwrap();
        match map.get("keyword") {
            Some(&Value::String(ref name)) => {
                keyword_param = name.to_string();
            }
            _ => keyword_param = "".to_string(),
        }
        match map.get("kind") {
            Some(&Value::String(ref name)) => {
                kind_param = name.to_string();
            }
            _ => kind_param = "all".to_string(),
        }
        match map.get("page") {
            Some(&Value::String(ref name)) => {
                page_param = name.to_string();
            }
            _ => page_param = "1".to_string(),
        }
    }

    let keyword_search = keyword_param.clone();
    let kind_search = kind_param.clone();
    let keyword_count = keyword_param.clone();
    let kind_count = kind_param.clone();

    let mut resp = Response::new();

    #[derive(Serialize, Debug)]
    struct Data {
        logged_in: bool,
        login_user: models::user::User,
        posts: Vec<models::post::Post>,
        current_page: i32,
        total_page: i32,
        next_page: i32,
        prev_page: i32,
        keyword: String,
        kind: String,
        kind_all_active: String,
        kind_post_active: String,
        kind_nippo_active: String,
    }

    let mut page = page_param.parse::<i32>().unwrap();
    let offset = (page - 1) * PAGINATES_PER;
    let limit = PAGINATES_PER;

    let posts: Vec<models::post::Post>;
    let count: i32;

    match models::post::search(&conn, &keyword_search, &kind_search, &offset, &limit) {
        Ok(posts_db) => {
            posts = posts_db;
        }
        Err(e) => {
            error!("Errored: {:?}", e);
            return Ok(Response::with((status::InternalServerError)));
        }
    }

    match models::post::search_count(&conn, &keyword_count, &kind_count) {
        Ok(count_db) => {
            count = count_db;
        }
        Err(e) => {
            error!("Errored: {:?}", e);
            return Ok(Response::with((status::InternalServerError)));
        }
    }

    if page == 0 {
        page = 1;
    }
    let mut kind_all_active = String::from("");
    let mut kind_post_active = String::from("");
    let mut kind_nippo_active = String::from("");
    let mui_is_active = String::from("mui--is-active");
    if kind_param == "all" {
        kind_all_active = mui_is_active.clone();
    }
    if kind_param == "post" {
        kind_post_active = mui_is_active.clone();
    }
    if kind_param == "nippo" {
        kind_nippo_active = mui_is_active.clone();
    }
    let data = Data {
        logged_in: login_id != 0,
        login_user: login_user,
        posts: posts,
        current_page: page,
        total_page: count / PAGINATES_PER + 1,
        next_page: page + 1,
        prev_page: page - 1,
        keyword: keyword_param,
        kind: kind_param,
        kind_all_active: kind_all_active,
        kind_post_active: kind_post_active,
        kind_nippo_active: kind_nippo_active,
    };

    resp.set_mut(Template::new("search", to_json(&data)))
        .set_mut(status::Ok);
    return Ok(resp);
}

pub fn tag_handler(req: &mut Request) -> IronResult<Response> {
    let conn = get_pg_connection!(req);
    let mut login_user: models::user::User = models::user::User{..Default::default()};
    match handlers::account::current_user(req, &conn) {
        Ok(user) => { login_user = user; }
        Err(e) => { error!("Errored: {:?}", e); }
    }
    let login_id = login_user.id;
    if login_id == 0 {
        return Ok(Response::with((status::Found, Redirect(helper::redirect_url("/signin")))));
    }

    let page_param: String;
    let tag_param: String;

    {
        use params::Params;
        let map = &req.get_ref::<Params>().unwrap();

        match helper::get_param(map, "page") {
            Ok(value) => page_param = value,
            Err(_) => page_param = String::from("1"),
        }

        match helper::get_param(map, "name") {
            Ok(value) => tag_param = value,
            Err(st) => return Ok(Response::with((st))),
        }
    }

    let mut resp = Response::new();

    #[derive(Serialize, Debug)]
    struct Data {
        logged_in: bool,
        login_user: models::user::User,
        posts: Vec<models::post::Post>,
        current_page: i32,
        total_page: i32,
        next_page: i32,
        prev_page: i32,
        tag_name: String,
    }

    let mut page = page_param.parse::<i32>().unwrap();
    let offset = ( page - 1 ) * PAGINATES_PER;
    let limit = PAGINATES_PER;

    let posts: Vec<models::post::Post>;
    let count: i32;

    match models::tag::tag_search(&conn, &tag_param, offset, limit) {
        Ok(posts_db) => {
            posts = posts_db;
        },
        Err(e) => {
            error!("Errored: {:?}", e);
            return Ok(Response::with((status::InternalServerError)));
        }
    }

    match models::tag::tag_count(&conn, &tag_param) {
        Ok(count_db) => {
            count = count_db;
        },
        Err(e) => {
            error!("Errored: {:?}", e);
            return Ok(Response::with((status::InternalServerError)));
        }
    }

    if page == 0 {
        page = 1;
    }
    let data = Data {
        logged_in: login_id != 0,
        login_user: login_user,
        posts: posts,
        current_page: page,
        total_page: count / PAGINATES_PER + 1,
        next_page: page + 1,
        prev_page: page - 1,
        tag_name: tag_param,
    };

    resp.set_mut(Template::new("tag", to_json(&data))).set_mut(status::Ok);
    return Ok(resp);
}
*/