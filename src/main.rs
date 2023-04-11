use yew::prelude::*;
use statusbar::StatusBar;
use crate::conf::Configuration;
use lister::SongList;

mod conf;
mod statusbar;
pub mod networking;
pub mod auth;
pub mod player_state;
pub mod song;
pub mod lister;

#[function_component]
fn App() -> Html{
    let conf = Configuration::new().unwrap();
    let status_og = use_state(|| None);
    let status = status_og.clone();
    {
        let conf = conf.clone();
        let client = reqwest::Client::new();
        use_effect(move || {
            let status = status.clone();
            wasm_bindgen_futures::spawn_local(async move{
                if let Ok(s) = networking::get_state(&conf, &client).await{
                    status.set(Some(s));
                }
            });
        });
    }
    let song_list = use_state(|| vec![]);
    {
        let client = reqwest::Client::new();
        let sl = song_list.clone();
        use_effect(move || {
            let sl = sl.clone();
            wasm_bindgen_futures::spawn_local(async move{
                if let Ok(r) = networking::get_song_list(&conf, &client).await{
                    sl.set(r);
                }
            })
        })
    }
    

    html!{
        <div>
            <StatusBar player_state={(*status_og).clone()} />
            <hr/>
            <h1>{"Songs: "}</h1>
            <SongList song_list={(*song_list).clone()}/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct Psp{
    player_state: Option<player_state::PlayerState>,
}

fn main() {
    yew::Renderer::<App>::new().render();
}

