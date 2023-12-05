use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct MyHttpResponse{
    pub code: i8,
    pub message: String
}