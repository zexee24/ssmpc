use crate::conf::Configuration;
use crate::song::Song;
use gloo_console::log;
use lister::SongList;
use networking::add_song_to_queue;
use search_bar::SearchBar;
use statusbar::StatusBar;
use yew::prelude::*;
use youtube::results::YoutubeResults;

pub mod auth;
mod conf;
pub mod lister;
pub mod networking;
pub mod player_state;
mod search_bar;
pub mod song;
mod statusbar;
mod youtube;

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
    #[allow(clippy::redundant_closure)]
    let song_list = use_state_eq(|| vec![]);
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
    let add_song = Callback::from(move |s: Song| {
        let conf = conf.clone();
        wasm_bindgen_futures::spawn_local(async move {
            add_song_to_queue(conf.clone(), &reqwest::Client::new(), vec![s.clone()]).await
        })
    });
    let search: UseStateHandle<AttrValue> = use_state(|| "".into());
    let se = search.clone();
    let on_change_search = Callback::from(move |s: AttrValue| {
        log!("{:?}", s.to_string());
        se.set(s)
    });
    html! {
        <div>
            <StatusBar player_state={(*status).clone()} />
            <hr/>
            <SearchBar oninput={on_change_search}/>
            <h1>{"Songs: "}</h1>
            <SongList song_list={(*song_list).clone() } on_click={add_song} search={(*search).clone()}/>
            <YoutubeResults videos={vec![]}/>
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
