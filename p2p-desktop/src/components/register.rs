use crate::{P2PAppMessage, P2PAppState};
use iced::{Element, Length};
use iced_widget::{Column, container, Container};
use crate::assistance::error::MyError;
use crate::assistance::http::MyHttpResponse;

pub struct RegisterState{
    pub name: String,
    pub password: String,
    pub confirmed_password: String
}

#[derive(Debug,Clone)]
pub enum RegisterMessage{
    Name(String),
    Password(String),
    ConfirmedPassword(String),
    SubmitRegister,
    RegisterResponse(Result<MyHttpResponse, MyError>),
    //Page
    BackLoginPage
}

pub fn register_view(p2p_app_state: &P2PAppState) -> Element<RegisterMessage> {
    Container::new(
        Column::new()
            .push(
                iced_widget::text_input(
                    "input name",
                    &p2p_app_state.register_state.name
                )
                    .on_input(RegisterMessage::Name)
                    .padding(10)
                    .width(Length::Fill)
            )
            .push(
                iced_widget::text_input(
                    "input password",
                    &p2p_app_state.register_state.password
                )
                    .password()
                    .on_input(RegisterMessage::Password)
                    .padding(10)
                    .width(Length::Fill)
            )
            .push(
                iced_widget::text_input(
                    "input confirmed password",
                    &p2p_app_state.register_state.confirmed_password
                )
                    .password()
                    .on_input(RegisterMessage::ConfirmedPassword)
                    .padding(10)
                    .width(Length::Fill)
            )
            .push(
                iced_widget::button("Register")
                    .on_press(RegisterMessage::SubmitRegister)
                    .padding(10)
                    .width(Length::Fill)
            )
            .push(
                iced_widget::button("Back Login")
                    .on_press(RegisterMessage::BackLoginPage)
                    .padding(10)
                    .width(Length::Fill)
            )
            .spacing(10)
    )
        .center_x()
        .center_y()
        .into()
}