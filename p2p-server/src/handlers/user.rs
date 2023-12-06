use actix_session::{Session, SessionInsertError};
use actix_web;
use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::web::Json;
use log;
use crate::AppState;
use crate::assistance::error::MyError;
use crate::assistance::http_response::MyHttpResponse;
use crate::model::metadata::METADATA;
use crate::model::user::User;

pub fn config(cfg: &mut web::ServiceConfig){
    cfg
        .service(web::resource("/login").route(web::post().to(login)))
        .service(web::resource("/register").route(web::post().to(register)))
        .service(web::resource("/show_metadata").route(web::get().to(show_metadata)))
        .service(web::resource("/delete_metadata").route(web::delete().to(delete_metadata)))
    ;
}

async fn login(session: Session, app_state: web::Data<AppState>, user_info: Json<User>) -> HttpResponse{
    log::info!("login");
    let pool = &app_state.backend_db;
    let user = User::new(&user_info.name, &user_info.password);
    return match user.check(pool).await {
        Ok(existed) => {
            if existed {
                if let (Ok(_), Ok(_)) = (session.insert("name", user.name), session.insert("password", user.password)) {
                    HttpResponse::Ok().json(MyHttpResponse::new(0, "successfully"))
                } else {
                    MyError::new("SessionInertError").to_http_response()
                }
            } else {
                HttpResponse::Ok().json(MyHttpResponse::new(1, "password wrong"))
            }
        },
        Err(my_error) => {
            my_error.to_http_response()
        }
    }
}
async fn register(app_state: web::Data<AppState>, user_info: Json<User>) -> HttpResponse{
    log::info!("register");
    let pool = &app_state.backend_db;
    let new_user = User::new(&user_info.name, &user_info.password);
    match new_user.insert(pool).await{
        Ok(_) => {
            HttpResponse::Ok().json(MyHttpResponse::new(0, "Ok"))
        },
        Err(my_error) => {
            my_error.to_http_response()
        }
    }
}
async fn show_metadata(session: Session, app_state: web::Data<AppState>) -> HttpResponse{
    match User::authorized_user(session){
        Ok(user) => {
            let pool = &app_state.backend_db;
            // ZIHAO: 对于有返回数据响应，直接返回数据，无code
            match user.show_metadata(pool).await {
                Ok(metadata_vec) => {
                    return HttpResponse::Ok().json(metadata_vec)
                },
                Err(my_error) => {
                    return my_error.to_http_response();
                }

            }

        },
        Err(http_response) => {
            return http_response;
        }

    }
}
async fn delete_metadata(session: Session, request: HttpRequest, app_state: web::Data<AppState>) -> HttpResponse{
    return match User::authorized_user(session) {
        Ok(user) => {
            let pool = &app_state.backend_db;
            let metadata_id = request.query_string().split('=').collect::<Vec<_>>()[1].parse::<i64>().unwrap();
            match METADATA::delete(metadata_id, pool).await {
                Ok(result) => {
                    if result {
                        HttpResponse::Ok().json(MyHttpResponse::new(0, "ok"))
                    } else {
                        HttpResponse::InternalServerError().json(MyHttpResponse::new(-1, "delete fail"))
                    }
                },
                Err(my_error) => {
                    my_error.to_http_response()
                }
            }
        },
        Err(http_response) => {
            http_response
        }
    }
}