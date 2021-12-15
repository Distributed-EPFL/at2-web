use yew::prelude::*;

#[function_component(Style)]
pub fn style() -> Html {
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
