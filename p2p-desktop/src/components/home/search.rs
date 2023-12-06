use iced::Element;
use iced_widget::Container;
use crate::assistance::metadata::METADATA;
use crate::P2PAppState;

pub struct SearchState{
    pub file_name_to_search: String,
    pub searched_metadata: Vec<METADATA>
}

#[derive(Clone, Debug)]
pub enum SearchMessage{
    // get
    GetNothing,
    // enter
    FileName(String),
    // submit
    SubmitSearch
}

pub fn search_view(app_state: &P2PAppState) -> Element<SearchMessage> {
    Container::new(
        iced_widget::text("search_view")
    )
        .into()
}