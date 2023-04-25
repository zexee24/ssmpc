use super::video::Video;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct YoutubeProps {
    pub videos: Vec<Video>,
    pub onclick: Callback<String>,
}
#[derive(Properties, PartialEq)]
struct VideoProps {
    video: Video,
    onclick: Callback<String>,
}

#[function_component]
pub fn YoutubeResults(YoutubeProps { videos, onclick }: &YoutubeProps) -> Html {
    videos
        .iter()
        .map(|v| html! {<VideoComponent video={v.clone()} onclick={onclick.clone()}/>})
        .collect()
}

#[function_component]
fn VideoComponent(VideoProps { video, onclick }: &VideoProps) -> Html {
    let v = video.clone();
    let onclick = onclick.clone();
    html! {
        <div>
            <hr/>
            <h3>{v.title}</h3>
            <h4>{v.artist}</h4>
            <button onclick={move |_|{
                onclick.emit(v.id.clone())
            }}> {"Download"} </button>
        </div>
    }
}
