use iced::{Element, Length};
use iced_widget::{Column, Container};
use crate::assistance::error::MyError;
use crate::assistance::http::MyHttpResponse;
use crate::assistance::metadata::METADATA;
use crate::P2PAppState;

pub struct UploadState {
    pub size_text: String,
    pub metadata_to_upload: METADATA
}

#[derive(Clone, Debug)]
pub enum UploadMessage{
    // get
    GetUploadedMetadata,
    // enter
    NAME(String),
    Size(String),
    Path(String),
    // Ip(String),
    // Port(String),
    // submit
    SubmitUpload,
    UploadResponse(Result<MyHttpResponse, MyError>)
}

pub fn upload_view(app_state: &P2PAppState) -> Element<UploadMessage>{
    Container::new(
        iced_widget::column(
            vec![
                iced_widget::text_input(
                    "file name",
                    &app_state.home_state.upload_state.metadata_to_upload.name
                )
                    .on_input(UploadMessage::NAME)
                    .into(),
                iced_widget::text_input(
                    "size",
                    &app_state.home_state.upload_state.size_text
                )
                    .on_input(UploadMessage::Size)
                    .into(),
                iced_widget::text_input(
                    "path",
                    &app_state.home_state.upload_state.metadata_to_upload.path
                )
                    .on_input(UploadMessage::Path)
                    .into(),
                // iced_widget::text_input(
                //     "ip",
                //     &app_state.home_state.upload_state.metadata_to_upload.ip
                // )
                //     .on_input(UploadMessage::Ip)
                //     .into(),
                // iced_widget::text_input(
                //     "port",
                //     &app_state.home_state.upload_state.metadata_to_upload.port
                // )
                //     .on_input(UploadMessage::Port)
                //     .into(),
                iced_widget::button("Upload")
                    .on_press(UploadMessage::SubmitUpload)
                    .width(Length::Fill)
                    .into()
            ]
        )
    )
        .into()
}