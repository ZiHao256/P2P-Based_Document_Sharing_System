use actix_session::Session;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use sqlx::{query, Row, SqlitePool};
use crate::assistance::error::MyError;
use crate::assistance::http_response::MyHttpResponse;
use crate::model::metadata::METADATA;

#[derive(Serialize, Deserialize)]
pub struct User{
    pub name: String,
    pub password: String
}

impl User{
    pub fn new (name: &str, password: &str) -> User {
        User{
            name: name.to_string(),
            password: password.to_string()
        }
    }
    pub fn authorized_user(session: Session) -> Result<User, HttpResponse>{
        if let (Ok(option_name), Ok(option_password)) = (session.get::<String>("name"), session.get::<String>("password")) {
            if let (Some(name), Some(password)) = (option_name, option_password) {
                Ok(User{
                    name,
                    password
                })
            } else {
                Err(HttpResponse::Unauthorized().json(MyHttpResponse::new(4, "empty session")))
            }
        } else {
            Err(HttpResponse::InternalServerError().json(MyHttpResponse::new(-1, "deserialize session")))
        }
    }
    pub async fn insert(&self, pool: &SqlitePool) -> Result<(), MyError>{
        query(&format!("insert into user(name, password) values(?,?);"))
            .bind(&(self.name))
            .bind(&(self.password))
            .execute(pool).await?;
        Ok(())
    }
    pub async fn check(&self, pool: &SqlitePool) -> Result<bool, MyError> {
        let result = query(&format!("select * from user where name = ?"))
            .bind(&self.name)
            .fetch_one(pool).await?;
        let password = result.get::<String, &str>("password");
        return Ok(password == self.password)
    }
    pub async fn show_metadata(&self, pool: &SqlitePool) -> Result<Vec<METADATA>, MyError> {
        let mut metadata_vec = vec![];
        let results = query("select * from metadata where user_name = ?")
            .bind(&self.name)
            .fetch_all(pool).await?;
        for result in results{
            metadata_vec.push(METADATA{
                id: result.get("id"),
                user_name: result.get("user_name"),
                name: result.get("name"),
                size: result.get("size"),
                path: result.get("path"),
                ip: result.get("IP"),
                port: result.get("PORT"),
            })
        }
        Ok(metadata_vec)
    }
}