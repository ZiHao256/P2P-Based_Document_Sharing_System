use iced::Element;
use iced_widget::Container;
use crate::assistance::metadata::METADATA;
use crate::P2PAppState;

pub struct UploadState{
    pub metadata_to_upload: METADATA
}

#[derive(Clone, Debug)]
pub enum UploadMessage{
    // get
    GetUploadedMetadata,
    // enter
    NAME(String),
    Size(f64),
    Path(String),
    Ip(String),
    Port(String),
    // submit
    SubmitUpload
}

pub fn upload_view(app_state: &P2PAppState) -> Element<UploadMessage>{
    Container::new(
        iced_widget::text("upload_view")
    )
        .into()
}