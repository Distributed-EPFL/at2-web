use yew::prelude::*;

pub struct Welcome;

impl Component for Welcome {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! { <>
            <h1> { "Welcome to AT2" } </h1>

            <p>
                <a href="https://factory.c4dt.org/showcase/at2"> { "AT2" } </a>
                { " is a research project developed at Prof. Rachid Guerraoui's " }
                <a href="https://dcl.epfl.ch"> { "Distributed Computing Lab" } </a>
                { ", at EPFL.
                It stands for Asynchronous Trustworthy Transfer, it is a
                new way to transfer assets (such as coins) throughout a
                network of potentially rogue participants.
                It is quite close to Bitcoin in its capabilities, while
                being orders of magnitudes faster and consuming much less
                energy.
                It achieves such speed by avoiding to synchronize a common
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
                First, you will create an account that will be credited
                with some initial assets. You can then use it to send money
                to another account on the network, as you would do on other
                distributed ledgers.
                To show that indeed AT2 is blazingly fast, the last part is a
                speed test where you can send as many transactions as you
                want and measure how well the network is handling the load.
            " } </p>
        </> }
    }
}
