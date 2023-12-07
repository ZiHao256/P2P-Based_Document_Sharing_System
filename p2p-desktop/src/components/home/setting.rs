use iced::{Element, Length};
use crate::assistance::error::MyError;
use crate::assistance::http::MyHttpResponse;
use crate::P2PAppState;

#[derive(Debug)]
pub struct SettingState{
    pub(crate) ip: String,
    pub(crate) port: String
}

#[derive(Clone, Debug)]
pub enum SettingMessage{
    // route
    RouteHere,
    // set
    SetIp(String),
    SetPort(String),
    SetSocket,
    // test
    Test,
    TestResponse(Result<MyHttpResponse, MyError>)

}

pub fn setting_view(app_state: &P2PAppState) -> Element<SettingMessage> {
    iced_widget::container(
        iced_widget::column(
            vec![
                iced_widget::text_input(
                    "enter ip",
                    &app_state.home_state.setting_state.ip
                )
                    .on_input(SettingMessage::SetIp)
                    .width(Length::Fill)
                    .into(),
                iced_widget::text_input(
                    "enter port",
                    &app_state.home_state.setting_state.port
                )
                    .on_input(SettingMessage::SetPort)
                    .width(Length::Fill)
                    .into(),
                iced_widget::button(
                    "test"
                )
                    .on_press(SettingMessage::Test)
                    .width(Length::Fill)
                    .into(),
                iced_widget::button(
                    "set"
                )
                    .on_press(SettingMessage::SetSocket)
                    .width(Length::Fill)
                    .into()
            ]
        )
    ).into()
}