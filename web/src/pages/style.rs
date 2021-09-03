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
            .bottom {
              position: fixed;
              bottom: 0;
              width: 100%;

              border-top: solid lightgrey;

              text-align: center;

              /* override base CSS */
              margin: 0;
            }
            .bottom > * {
              margin: 1em;
            }

            .page {
                text-align: center;

                width: 40em;
                margin: 10em auto;
            }
            .page > * {
                margin: 2em auto;
            }
            .page > p {
                text-align: justify;
            }
        " } </style> }
    }
}
