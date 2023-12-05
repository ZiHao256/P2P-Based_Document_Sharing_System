use log::info;
use serde::Serialize;
use crate::assistance::http::MyHttpResponse;
use crate::{BASE_URL, P2PAppMessage, P2PAppState};
use crate::assistance::error::MyError;

#[derive(Default, Serialize, Clone)]
pub struct User {
    pub name: String,
    pub password: String,
}

impl User {

}