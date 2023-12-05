use iced::{Alignment, Element, Length};
use iced_widget::{button, Column, container, Container, Row, text};
use crate::{P2PAppMessage, P2PAppState};

#[derive(Clone, Debug)]
pub enum MenuItem{
    File,
    Upload,
    Search,
    Setting
}

pub struct HomeState{
    pub file_button: button::State,
    pub upload_button: button::State,
    pub search_button: button::State,
    pub setting_button: button::State
}

#[derive(Clone, Debug)]
pub enum HomeMessage{
    MenuItemSelected(MenuItem)
}

pub fn home_view(app_state: &P2PAppState) -> Element<HomeMessage> {
    Container::new(
        Row::new()
            .push(
                iced_widget::Button::new(
                    "file"
                ).on_press(HomeMessage::MenuItemSelected(MenuItem::File))
                    .width(Length::Fill)
            )
            .push(
                iced_widget::Button::new(
                    "upload"
                ).width(Length::Fill)
            )
            .push(
                iced_widget::Button::new(
                    "search"
                ).width(Length::Fill)
            )
            .push(
                iced_widget::Button::new(
                    "setting"
                ).width(Length::Fill)
            ).width(Length::Fill)

    ).into()
}