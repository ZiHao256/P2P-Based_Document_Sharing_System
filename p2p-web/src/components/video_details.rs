use yew::prelude::*;
use crate::components::videos_list::Video;

#[derive(Properties, PartialEq)]
pub struct VideoDetailProps {
    pub video: Video
}

#[function_component(VideoDetails)]
pub fn video_details(VideoDetailProps {video}: &VideoDetailProps) -> Html {
    html!{
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}