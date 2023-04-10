use yew::prelude::*;
use statusbar::StatusBar;
use crate::conf::Configuration;

mod conf;
mod statusbar;
pub mod networking;
pub mod auth;
pub mod player_state;
pub mod song;

#[function_component]
fn App() -> Html{
    let conf = Configuration::new().unwrap();
    let status_og = use_state(|| None);
    let status = status_og.clone();
    use_effect(move || {
        let status = status.clone();
        wasm_bindgen_futures::spawn_local(async move{
            if let Ok(s) = networking::get_state(conf).await{
                status.set(Some(s));
            }
        });
    });
    

    html!{
        <div>
            <StatusBar player_state={(*status_og).clone()} />
            <h1>{"Hello, World"}</h1>
            <p>{"line 2"}</p>
            <button type="button">{"Klikkaa Tästä!"}</button>
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

