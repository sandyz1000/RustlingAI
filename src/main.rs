mod api;
mod components;
mod hooks;
mod store;
mod types;
mod utils;
mod constants;

use yew::prelude::*;


#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <p>{ "Welcome to yew!" }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
