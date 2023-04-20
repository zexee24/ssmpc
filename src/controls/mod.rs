use messages::Message;
use yew::prelude::*;

use crate::player_state::PlayerState;

pub mod messages;

#[function_component]
pub fn ControlPanel(
    ControlPanelProps {
        on_click,
        player_state,
    }: &ControlPanelProps,
) -> Html {
    let on_click = on_click.clone();
    html! {
        <div>
            {
                match player_state.paused{
                    true => {
                        let on_click = on_click.clone();
                        html!{
                        <button onclick={move |_| on_click.emit(Message::Play)}>
                        {"Play"}</button>}
                    },
                    false => {
                        let on_click = on_click.clone();
                        html!{
                        <button onclick={move |_| on_click.emit(Message::Pause)}>
                        {"Pause"}</button>}
                    }
                }
            }
            <button onclick={move |_| on_click.emit(Message::Skip(vec![0]))}>{"Skip"}</button>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ControlPanelProps {
    pub on_click: Callback<Message>,
    pub player_state: PlayerState,
}
