use yew::prelude::*;
use super::video::Video;

#[derive(Properties, PartialEq)]
pub struct YoutubeProps {
    pub videos: Vec<Video>,
}
#[derive(Properties, PartialEq)]
struct VideoProps {
    video: Video,
}

#[function_component]
pub fn YoutubeResults(YoutubeProps { videos }: &YoutubeProps) -> Html {
    videos
        .iter()
        .map(|v| html! {<VideoComponent video={v.clone()}/>})
        .collect()
}

#[function_component]
fn VideoComponent(VideoProps { video }: &VideoProps) -> Html {
    let v = video.clone();
    html! {
        <div>
            <p>{v.title}</p>
            <p>{v.artist}</p>
        </div>
    }
}
