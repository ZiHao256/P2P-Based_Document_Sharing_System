#[derive(Clone, Debug)]
pub enum MyError{
    ClientError{code: i8, message: String},
    ServerError{code: i8, message: String},
    PeerError{code: i8, message: String}
}

impl MyError{
    pub fn new(code: i8, message: String) {

    }
    pub fn get_message(&self) -> String{
        format!("{self:?}")
    }
}
