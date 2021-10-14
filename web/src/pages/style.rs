use yew::prelude::*;

pub struct Style;

impl Component for Style {
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
        html! { <style> { "
            body {
                font-family: Roboto, sans-serif;
                margin-left: 0px;
            }

            .boxes {
                display: flex;
                align-items: center;
                justify-content: space-around;
            }
            .boxes > p {
                border: solid;
                padding: 1em 2em;
            }
        " } </style> }
    }
}
