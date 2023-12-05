use iced_widget::{column, Column, container, Container};
use crate::{P2PAppMessage, P2PAppState};
use crate::components::page;
use iced::{Element, Length};
use crate::assistance::error::MyError;
use crate::assistance::http::MyHttpResponse;

#[derive(Clone)]
pub struct LoginState{
        pub name: String,
        pub password: String,
}

#[derive(Clone, Debug)]
pub enum LoginMessage{
    Name(String),
    Password(String),
    SubmitLogin,
    LoginResponse(Result<MyHttpResponse, MyError>),
    // Page
    EnterRegisterPage
}

pub fn login_view(p2p_app_state: &P2PAppState) -> Element<LoginMessage> {
    Container::new(
        Column::new()
            .push(
                iced_widget::text_input("input name", &p2p_app_state.login_state.name)
                    .on_input(LoginMessage::Name)
                    .padding(10)
                    .width(Length::Fill)
            )
            .push(
                iced_widget::text_input("input password", &p2p_app_state.login_state.password)
                    .on_input(LoginMessage::Password)
                    .padding(10)
                    .width(Length::Fill)
            )
            .push(
                iced_widget::button("Login").on_press(LoginMessage::SubmitLogin)
                    .padding(10)
                    .width(Length::Fill)
            )
            .push(
                iced_widget::button("Register")
                    .on_press(LoginMessage::EnterRegisterPage)
                    .padding(10)
                    .width(Length::Fill)
            )
            .spacing(10)
    )
        .center_x()
        .center_y()
        // ZIHAO: Container<LoginMessage> -> Element<LoginMessage>
        .into()

}