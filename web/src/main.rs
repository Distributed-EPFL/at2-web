use yew::prelude::*;

mod agents;
mod config;
mod pages;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    html! { <>
        <style> { include_str!(concat!(env!("OUT_DIR"), "/c4dt.css")) } </style>

        <pages::Style/>

        <pages::Pages/>
    </> }
}
