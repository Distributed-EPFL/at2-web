use yew::prelude::*;

/// Shared CSS declarations
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
                margin-left: 0px;
            }

            * {
                font-family: Roboto, sans-serif;
                font-weight: 400;
                letter-spacing: 0.25px;
            }
            p {
                font-size: 14px;
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
