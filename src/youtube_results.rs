use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct YoutubeProps {
    pub videos: Vec<AttrValue>,
}
#[derive(Properties, PartialEq)]
pub struct VideoProps {
    video: AttrValue,
}

#[function_component]
pub fn YoutubeResults(YoutubeProps { videos }: &YoutubeProps) -> Html {
    videos
        .iter()
        .map(|v| html! {<VideoComponent video={v}/>})
        .collect()
}

#[function_component]
fn VideoComponent(VideoProps { video }: &VideoProps) -> Html {
    html! {
        <div>
            <p>{video}</p>
        </div>
    }
}
