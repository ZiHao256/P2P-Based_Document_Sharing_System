pub mod handlers;
pub mod assistance;
pub mod model;

use actix_cors::Cors;
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_web::{self, web, middleware, HttpResponse};
use actix_web::cookie::Key;
use env_logger;
use env_logger::Env;
use log;
use sqlx;
use dotenv;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub struct AppState{
    pub backend_db: sqlx::SqlitePool
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    // init dot env
    dotenv::dotenv().ok();

    // init logger middle
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // init session

    //init cors

    // int db connection pool
    let app_state = actix_web::web::Data::new(AppState{
        backend_db: sqlx::SqlitePool::connect(&format!("sqlite://{}",dotenv::var("BACKEND_DB").unwrap())).await.unwrap()
    });

    actix_web::HttpServer::new(move ||{
        actix_web::App::new()
            .wrap(middleware::Logger::default())
            .wrap(SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                .cookie_secure(false)
                .build())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600)
            )
            .app_data(app_state.clone())
            .service(web::scope("/user").configure(handlers::user::config))
            .service(web::scope("/metadata").configure(handlers::metadata::config))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
