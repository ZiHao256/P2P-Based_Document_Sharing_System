use std::process::Command;
use iced_widget::{column, Column, container, Container};
use crate::{P2PAppMessage, P2PAppState};
use crate::components::page;
use iced::{Alignment, Element, Length};
use iced::alignment::{Horizontal, Vertical};
use iced_widget::container::StyleSheet;
use iced_widget::core::Widget;
use tokio::task::LocalEnterGuard;
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
                iced_widget::text("Login")
                    .size(50)
                    .height(150)
                    .vertical_alignment(Vertical::Center)
            )
            .push(
                iced_widget::column(
                    vec![
                        iced_widget::text_input("input name", &p2p_app_state.login_state.name)
                            .on_input(LoginMessage::Name)
                            .padding(10)
                            .width(Length::Fill)
                            .into(),
                        iced_widget::text_input("input password", &p2p_app_state.login_state.password)
                            .password()
                            .on_input(LoginMessage::Password)
                            .padding(10)
                            .width(Length::Fill)
                            .into()
                    ]
                )
                    .spacing(10)
            )
            .push(
                iced_widget::column(
                    vec![
                        iced_widget::button("Login").on_press(LoginMessage::SubmitLogin)
                            .padding(10)
                            .width(Length::Fill)
                            .into(),
                        iced_widget::button("Register")
                            .on_press(LoginMessage::EnterRegisterPage)
                            .padding(10)
                            .width(Length::Fill)
                            .into()
                    ]
                )
                    .spacing(10)
            )
            .spacing(50)
            .padding(100)
            // ZIHAO: 用于Column和Row元素
            .align_items(Alignment::Center)
    )
        // ZIHAO: Container<LoginMessage> -> Element<LoginMessage>
        .into()

}