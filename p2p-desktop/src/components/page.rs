#[derive(Clone, Debug)]
pub enum PageState{
    LoginPage,
    RegisterPage,
    HomePage
}

#[derive(Debug, Clone)]
pub enum PageMessage{
    SwitchToHomePage(String),
    SwitchToRegisterPage,
    SwitchToLoginPage(String)
}