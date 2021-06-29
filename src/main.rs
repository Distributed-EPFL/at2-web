use yew::prelude::*;

mod pages;

fn main() {
    yew::start_app::<App>();
}

struct App {}

impl Component for App {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! { <>
            <pages::Style/>
            <pages::Pages />
        </> }
    }
}
