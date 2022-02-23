use yew::prelude::*;

pub struct Welcome;

impl Component for Welcome {
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
            <h1> { "Welcome to AT2" } </h1>

            <p>
                <a href="https://factory.c4dt.org/showcase/at2" target="_blank">
                    { "Asynchronous Trustworthy Transfer" }
                </a>
                { " (AT2) is a research project developed at
                    Prof. Rachid Guerraoui's " }
                <a href="https://dcl.epfl.ch" target="_blank"> {
                    "Distributed Computing Lab"
                } </a>
                { ", at EPFL. It is a new way to " }
                <b>{ "transfer assets" }</b>
                { " (such as coins) throughout a network of potentially
                    rogue participants. It is quite " }
                <b>{ "close to Bitcoin" }</b>
                { " in its capabilities, while being orders of magnitudes " }
                <b>{ "faster" }</b> { " and consuming much " }
                <b>{ "less energy" }</b> { "." }
            </p>

            <p> { "
                AT2 achieves such speed by avoiding to synchronize a common
                state, the global consensus, in favor of using a local
                consensus, that usually isn't the same at every node but does
                converge to a common state.
                This way, transactions can be processed as soon as these are
                happening, not when the whole network agrees on what the
                correct ordering is.
            " } </p>

            <p> { "
                This demonstrator will take you through a classic account
                creation as you would do on another blockchain, but there
                is a twist at the end.
                First, you will " }
                <b>{ "create an account" }</b>
                { " that will be credited with some initial assets.
                You can then use it to " }
                <b>{ "send money" }</b>
                { " to another account on the network, as you would do on other
                distributed ledgers.
                To show that indeed AT2 is blazingly fast, the last part is a "
                } <b>{ "speed test" }</b>
                { " where you can send as many transactions as you
                want and measure how well the network is handling the load.
            " } </p>

            <img src="yacy-network.png" />
        </> }
    }
}
