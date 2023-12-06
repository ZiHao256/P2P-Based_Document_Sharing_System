pub mod upload;
pub mod file;
pub mod search;

use iced::{Alignment, Element, Length};
use iced_widget::{button, Column, container, Container, Row, text};
use crate::{P2PAppMessage, P2PAppState};
use crate::components::home::file::{file_view, FileMessage, FileState};
use crate::components::home::search::{search_view, SearchMessage, SearchState};
use crate::components::home::upload::{upload_view, UploadMessage, UploadState};

pub struct HomeState{
    pub current_child_page: u8,
    pub file_state: FileState,
    pub upload_state: UploadState,
    pub search_state: SearchState,
}

#[derive(Clone, Debug)]
pub enum HomeMessage{
    FileMessage(file::FileMessage),
    UploadMessage(upload::UploadMessage),
    SearchMessage(search::SearchMessage)
}

pub fn home_view(app_state: &P2PAppState) -> Element<HomeMessage> {
    // ZIHAO: 通过判断P2PAppState::HomeState::current_child_page来控制子组件的展示
    let child_view = match app_state.home_state.current_child_page{
        0 => {
            file_view(app_state)
                .map(HomeMessage::FileMessage)
        },
        1 => {
            upload_view(app_state)
                .map(HomeMessage::UploadMessage)
        },
        2 => {
            search_view(app_state)
                .map(HomeMessage::SearchMessage)
        },
        _ => {
            panic!()
        }
    };
    Container::new(
        Column::new()
            .push(
                Row::new()
                    .push(
                        iced_widget::Button::new(
                            "file"
                        ).on_press(HomeMessage::FileMessage(file::FileMessage::GetMetadata))
                            .width(Length::Fill)
                    )
                    .push(
                        iced_widget::Button::new(
                            "upload"
                        ).on_press(HomeMessage::UploadMessage(upload::UploadMessage::GetUploadedMetadata))
                            .width(Length::Fill)
                    )
                    .push(
                        iced_widget::Button::new(
                            "search"
                        ).on_press(HomeMessage::SearchMessage(search::SearchMessage::GetNothing))
                            .width(Length::Fill)
                    )
            )
            .push(
                child_view
            )

    ).into()
}