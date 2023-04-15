use ::yew::prelude::*;
use ::yew::Properties;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct SearchBarProps {
    pub oninput: Callback<AttrValue>,
}

#[function_component]
pub fn SearchBar(SearchBarProps { oninput }: &SearchBarProps) -> Html {
    let oninput = oninput.clone();
    html!(
        <div>
        <input oninput={move |e: InputEvent| {
            if let Some(t) = e.target(){
                let h: HtmlInputElement = t.dyn_into().unwrap();
                oninput.emit(h.value().into());
            }
        }} type="text" placeholder={"Search"}/>
        </div>
    )
}
