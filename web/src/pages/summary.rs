use yew::prelude::*;

pub struct Summary;

impl Component for Summary {
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
            <h1> { "Summary" } </h1>

            <p> { "We showed that AT2 is faster than other well known
                   blockchains. It achieves that speed by avoiding to
                   sync a common state, the global consensus. Most
                   blockchains are still using this method, but AT2's
                   paper demonstrates that it is unneeded to transfer
                   assets inside a network."
            } </p>

            <p> { "If you are interested in exploring this project more,
                   look at " }
                <a href="https://factory.c4dt.org/showcase/at2">
                    { "the project page" }
                </a>
                { "." }
            </p>
        </> }
    }
}
