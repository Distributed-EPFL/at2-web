use yew::prelude::*;

mod agents;
mod config;
mod pages;

fn main() {
    yew::start_app::<App>();
}

struct App;

impl Component for App {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! { <>
            <style> { include_str!(concat!(env!("OUT_DIR"), "/c4dt.css")) } </style>

            <pages::Style/>

            <pages::Pages/>
        </> }
    }
}
