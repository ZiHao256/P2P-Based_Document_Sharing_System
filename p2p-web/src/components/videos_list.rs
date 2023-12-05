use yew::prelude::*;
use serde::Deserialize;

#[function_component(VideosList)]
pub fn videos_list(VideosListProps{videos, on_click}: &VideosListProps) -> Html{
    let on_click = on_click.clone();
    videos.iter().map(|video| {
        let on_video_select = {
            let on_click = on_click.clone();
            let video = video.clone();
            Callback::from(move |_| {
                on_click.emit(video.clone())
            })
        };
        html! {
            <p key={video.id} onclick = {on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
        }
    }).collect()
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Video {
    pub(crate) id: usize,
    pub(crate) title: String,
    pub(crate) speaker: String,
    pub(crate) url: String,
}

#[derive(Properties, PartialEq)]
pub struct VideosListProps{
    pub videos: Vec<Video>,
    pub on_click: Callback<Video>
}