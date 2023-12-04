use actix_session::{Session, SessionInsertError};
use actix_web::HttpResponse;
use serde::Serialize;
use sqlx;

#[derive(Serialize)]
pub enum MyError{
    InternalError{code: i8, message: String},
    ValidationError{code: i8, message: String}
}

impl MyError{
    pub fn new(message: &str) -> MyError {
        MyError::InternalError {
            code: -1,
            message: message.to_string()
        }
    }

    pub fn to_http_response(self) -> HttpResponse{
        match &self {
            MyError::InternalError{code, message} => {
                HttpResponse::InternalServerError().json(self)
            },
            MyError::ValidationError {code, message} => {
                HttpResponse::Unauthorized().json(self)
            }
        }
    }
}

impl From<sqlx::Error> for MyError{
    fn from(value: sqlx::Error) -> Self {
        match value{
            sqlx::Error::Database(e) => {
                MyError::ValidationError {
                    code: 4,
                    message: e.to_string()
                }
            },
            _ => {
                MyError::InternalError {
                    code: -1,
                    message: value.to_string()
                }
            }
        }

    }
}

impl From<SessionInsertError> for MyError{
    fn from(value: SessionInsertError) -> Self {
        MyError::InternalError {
            code: -1,
            message: value.to_string()
        }
    }
}