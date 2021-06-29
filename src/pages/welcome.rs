use yew::prelude::*;

pub struct Welcome {}

impl Component for Welcome {
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
        html! { <div class=classes!("page")>
            <h1> { "Welcome to AT2" } </h1>

            <p> { "
                AT2 means Asynchronous Trustworthy Transfer, it is a new way
                to transfer assets (such as coins) throughout a network of
                potentially rogues participants.
                It is quite close to Bitcoin in its capabilities, while
                being order of magnitudes faster and consuming way less
                energy.
            " } </p>

            <p> { "
                In this demonstrator, you will first create an account
                that will be credited with some initial asset.
                You will then use it to send money to some other account on
                the network, as you would do in a classic blockchain.
                To show that indeed AT2 is blazing fast, the last part is
                a speed test where you will send as many transaction as you
                want and measure how well the network is handling the load.
            " } </p>
        </div> }
    }
}
