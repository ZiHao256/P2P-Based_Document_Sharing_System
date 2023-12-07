use std::fmt::format;
use iced::{Element, Length};
use iced_widget::{Column, Container, Row};
use iced_widget::core::Widget;
use crate::assistance::error::MyError;
use crate::assistance::metadata::METADATA;
use crate::components::home::search;
use crate::P2PAppState;

pub struct SearchState{
    pub file_name_to_search: String,
    pub searched_metadata: Vec<(METADATA, bool)>
}

#[derive(Clone, Debug)]
pub enum SearchMessage{
    // get
    RouteHere,
    // enter
    EnterFileName(String),
    // submit
    SubmitSearch,
    SearchResponse(Result<Vec<(METADATA, bool)>, MyError>),
    // download
    Download(METADATA),
    DownloadResponse(Result<String, MyError>)
}

pub fn search_view(app_state: &P2PAppState) -> Element<SearchMessage> {

    let header_view: Element<SearchMessage> = iced_widget::row(
        vec![
            iced_widget::text("file name").width(Length::Fill).into(),
            iced_widget::text("size").width(Length::Fill).into(),
            iced_widget::text("path").width(Length::Fill).into(),
            iced_widget::text("user name").width(Length::Fill).into(),
            iced_widget::text("ip").width(Length::Fill).into(),
            iced_widget::text("port").width(Length::Fill).into(),
            iced_widget::text("state").width(Length::Fill).into(),
            iced_widget::text("operation").width(Length::Fill).into()
        ]
    )
        .into()
        ;



    let metadata_view: Vec<Element<SearchMessage>>  = app_state.home_state.search_state.searched_metadata
        .iter().map( |(metadata, state)|{
        Row::new()
            .push(iced_widget::text(&metadata.name).width(Length::Fill))
            .push(iced_widget::text(&metadata.size).width(Length::Fill))
            .push(iced_widget::text(&metadata.path).width(Length::Fill))
            .push(iced_widget::text(&metadata.user_name).width(Length::Fill))
            .push(iced_widget::text(&metadata.ip).width(Length::Fill))
            .push(iced_widget::text(&metadata.port).width(Length::Fill))
            .push(iced_widget::text(state).width(Length::Fill))
            .push(
                if *state {
                    iced_widget::button(
                        "download"
                    )
                        .width(Length::Fill)
                        .on_press(search::SearchMessage::Download(metadata.clone()))
                } else {
                    iced_widget::button({
                        "download"
                    })
                        .width(Length::Fill)
                }
            )
            .into()
    })
        .collect()
        ;

    Container::new(
        Column::new()
            .push(
                iced_widget::text_input(
                    "Enter file name",
                    &app_state.home_state.search_state.file_name_to_search
                )
                    .width(Length::Fill)
                    .on_input(SearchMessage::EnterFileName)
            )
            .push(
                iced_widget::button(
                    "Search"
                )
                    .width(Length::Fill)
                    .on_press(SearchMessage::SubmitSearch)
            )
            .push(
                header_view
            )
            .push(
                iced_widget::column(
                    metadata_view
                )
            )
    ).into()
}

pub async fn test_online(ip: String, port: String) -> bool {
    return if let Ok(mut _stream) = tokio::net::TcpStream::connect(format!("{}:{}", ip, port)).await {
        true
    } else {
        false
    }
}