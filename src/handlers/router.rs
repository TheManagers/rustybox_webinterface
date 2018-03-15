use Router;
use handlers::index;
use handlers::login;
use handlers::middleware::AuthorizationCheck;
use handlers::middleware::SelectiveMiddleWare2;

pub fn create_router() -> Router {
    let mut router = Router::new();
    
    router.get("/", index::index_handler, "index");
    router.get("/newapp", SelectiveMiddleWare2::new(index::newapp_handler, vec!(AuthorizationCheck)), "newapp");
    router.post("/upload", SelectiveMiddleWare2::new(index::upload_handler, vec!(AuthorizationCheck)), "upload");

    router.get("/greet", login::greet, "greet");
    router.get("/login", login::login, "login");
    router.post("/login", login::login_post, "login_post");
    router.get("/logout", login::logout, "logout");

    return router;
}