use crate::conf::Configuration;
use crate::song::Song;
use crate::statusbar::Queue;
use controls::{messages::Message, ControlPanel};
use lister::SongList;
use networking::add_song_to_queue;
use search_bar::SearchBar;
use statusbar::StatusBar;
use yew::prelude::*;
use youtube::results::YoutubeResults;
use youtube::video::Video;

pub mod auth;
mod conf;
mod controls;
pub mod lister;
pub mod networking;
pub mod player_state;
mod search_bar;
pub mod song;
mod statusbar;
pub mod youtube;

#[function_component]
fn App() -> Html {
    let conf = Configuration::new().unwrap();
    let status = use_state(|| None);
    {
        let status = status.clone();
        let conf = conf.clone();
        let client = reqwest::Client::new();
        use_effect(move || {
            let status = status.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(s) = networking::get_state(&conf, &client).await {
                    status.set(Some(s));
                }
            });
        });
    }
    let song_list = use_state_eq(Vec::new);
    {
        let conf = conf.clone();
        let client = reqwest::Client::new();
        let sl = song_list.clone();
        use_effect(move || {
            let sl = sl.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(r) = networking::get_song_list(&conf, &client).await {
                    sl.set(r);
                }
            });
        });
    }
    let conft = conf.clone();
    let add_song = Callback::from(move |s: Song| {
        let conf = conft.clone();
        wasm_bindgen_futures::spawn_local(async move {
            add_song_to_queue(conf.clone(), &reqwest::Client::new(), vec![s.clone()]).await
        })
    });
    let search: UseStateHandle<AttrValue> = use_state(|| "".into());
    let se = search.clone();
    let on_change_search = Callback::from(move |s: AttrValue| se.set(s));
    let yt_list = use_state_eq(Vec::<Video>::new);
    let s = search.clone();
    let y = yt_list.clone();
    let conft = conf.clone();
    let search_on_click = move |_| {
        let conf = conft.clone();
        let s = s.clone();
        let y = y.clone();
        wasm_bindgen_futures::spawn_local(async move {
            y.set(
                networking::get_youtube_videos(&conf, s.to_string(), &reqwest::Client::new()).await,
            )
        })
    };
    let download_song = Callback::from(move |s: String| {
        let conf = Configuration::new().unwrap();
        wasm_bindgen_futures::spawn_local(async move {
            networking::download_video(&conf.clone(), &reqwest::Client::new(), s).await
        })
    });
    let control = Callback::from(move |m: Message| {
        let conf = conf.clone();
        wasm_bindgen_futures::spawn_local(async move {
            networking::control(&conf, &reqwest::Client::new(), m).await
        })
    });

    html! {
        <div>
            <StatusBar player_state={(*status).clone()} />
            {
                if let Some(s) = (*status).clone(){
                    html!{<ControlPanel player_state={s} on_click={control}/>}
                } else{html!{}}
            }
            <Queue player_state ={(*status).clone()}/>
            <hr/>
            <SearchBar oninput={on_change_search}/>
            <button onclick={search_on_click}>{"Search"}</button>
            <h2>{"Songs: "}</h2>
            <SongList song_list={(*song_list).clone() } on_click={add_song} search={(*search).clone()}/>
            <hr/>
            <h2>{"Search:"}</h2>
            <YoutubeResults videos={(*yt_list).clone()} onclick={download_song}/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct Psp {
    player_state: Option<player_state::PlayerState>,
}

fn main() {
    yew::Renderer::<App>::new().render();
}
