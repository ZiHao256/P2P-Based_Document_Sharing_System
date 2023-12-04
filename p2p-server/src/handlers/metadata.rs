use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::error::PayloadError::Http2Payload;
use log::info;
use crate::AppState;
use crate::assistance::http_response::MyHttpResponse;
use crate::model::user::User;
use crate::model::metadata::METADATA;

pub fn config(cfg: &mut web::ServiceConfig){
    cfg
        .service(web::resource("/upload").route(web::post().to(upload_metadata)))
        .service(web::resource("/lookup").route(web::get().to(lookup_metadata)))
    ;
}

pub async fn upload_metadata(session: Session, app_state: web::Data<AppState>, metadata: web::Json<METADATA>) -> HttpResponse {
    match User::authorized_user(session) {
        Ok(user) => {
            if user.name.eq(&metadata.user_name) {
                info!("user.name {}, metadata.user_name {}", user.name, metadata.user_name);
                let pool = &app_state.backend_db;
                match metadata.insert(pool).await {
                    Ok(_) => {
                        HttpResponse::Ok().json(MyHttpResponse::new(0, "ok"))
                    },
                    Err(my_error) => {
                        return my_error.to_http_response();
                    }
                }
            } else {
                HttpResponse::InternalServerError().json(MyHttpResponse::new(-1, "user_name != metadata.username"))
            }
        },
        Err(http_response) => {
            return http_response;
        }
    }
}
pub async fn lookup_metadata(session: Session, request: HttpRequest, app_state: web::Data<AppState>) -> HttpResponse{
    match User::authorized_user(session) {
        Ok(user) => {
            let pool = &app_state.backend_db;
            let file_name = request.query_string().split('=').collect::<Vec<_>>()[1];
            info!("param: {}, file_name: {}", request.query_string(), file_name);
            return match METADATA::lookup(file_name, pool).await {
                Ok(metadata_vec) => {
                    HttpResponse::Ok().json(metadata_vec)
                },
                Err(my_error) => {
                    my_error.to_http_response()
                }
            }
        },
        Err(http_response) => {
            return http_response;
        }
    }
}