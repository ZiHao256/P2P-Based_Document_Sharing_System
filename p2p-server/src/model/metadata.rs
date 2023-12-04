use log::info;
use serde::{Deserialize, Serialize};
use sqlx::{query, Row, SqlitePool};
use crate::assistance::error::MyError;

#[derive(Deserialize, Serialize)]
pub struct METADATA{
    pub id: i64,
    pub user_name: String,
    pub(crate) name: String,
    pub(crate) size: f64,
    pub(crate) path: String,
    pub(crate) ip: String,
    pub(crate) port: String,
}

impl METADATA {
    pub async fn insert(&self, pool: &SqlitePool) -> Result<(), MyError> {
        query(&format!(
            "insert into metadata(user_name, name, size, path, ip, port) values (?, ?, ?, ?, ?, ?)",
        )).bind(&self.user_name)
            .bind(&self.name)
            .bind(self.size)
            .bind(&self.path)
            .bind(&self.ip)
            .bind(&self.port)
            .execute(pool).await?;

        Ok(())
    }
    pub async fn lookup(file_name: &str, pool: &SqlitePool) -> Result<Vec<METADATA>, MyError > {
        let mut metadata_vec = vec![];
        let results = query("select * from metadata where name like ?;")
            .bind(format!("%{}%",file_name))
            .fetch_all(pool)
            .await.unwrap();
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
    pub async fn delete(metadata_id: i64, pool: &SqlitePool) -> Result<bool, MyError> {
        let result = query("delete from metadata where id = ?")
            .bind(metadata_id)
            .execute(pool)
            .await?;
        info!("delete result: {:?}", result);
        return if result.rows_affected() == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}