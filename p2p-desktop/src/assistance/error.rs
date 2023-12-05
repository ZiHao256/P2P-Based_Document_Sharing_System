#[derive(Clone, Debug)]
pub enum MyError{
    ClientError{code: i8, message: String},
    ServerError{code: i8, message: String},
    PeerError{code: i8, message: String}
}
