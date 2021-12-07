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
                line-height: 1.25rem;
                font-weight: 400;
            }
            p, td {
                font-size: 0.875rem;
            }

            @media (prefers-color-scheme: dark) {
                html {
                  --mdc-theme-primary: black;
                  --mdc-theme-surface: var(--dark-grey);
                  --mdc-theme-text-primary-on-background: var(--light-grey);
                }
            }
        " } </style> }
    }
}
