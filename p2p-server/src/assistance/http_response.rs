use serde;

#[derive(serde::Serialize)]
pub struct MyHttpResponse{
    code: i8,
    message: String
}

impl MyHttpResponse{
    pub fn new(code: i8, message: &str) -> MyHttpResponse{
        MyHttpResponse{
            code,
            message: message.to_string()
        }
    }
}