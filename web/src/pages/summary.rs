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
            <h1> { "Further informations" } </h1>

            <p> { "
                We showed that AT2 is behaving as any other blockchain.
                It can send asset, show other accounts on the network
                and watch the latest processed transactions.
                But using a speedtest reveals the true power of AT2:
                it is faster than other two most popular blockchains, beating " }
                <a href="https://bitcoin.org"> { "Bitcoin" } </a>
                { " and " }
                <a href="https://ethereum.org"> { "Ethereum" } </a>
                { " by a factor of 10.
                It achieves that speed by avoiding to synchronize a common state,
                the global consensus. It uses a local one, allowing the network
                to validate transactions before checking them with every node.
            " } </p>

            <p> { "
                As the transaction time is so short and because it can run in a
                browser, one can imagine using it directly as a means of
                payment, having a little button \"pay with AT2\" next to the
                credit cards.
                It can also be used as a means of communication, an
                instant chat ensuring that no rogue participant can interfere
                or reorder the messages.
            " } </p>

            <p> { "
                Most blockchains still uses the global consensus method,
                but " }
                <a href="https://arxiv.org/abs/1812.10844">
                    { "AT2's research paper" }
                </a> { " demonstrates that this is unnecessary to transfer asset
                within a network, one only need local consensus to do so.
                For the sake of transparency, there is another project based
                on a similar technology, " }
                <a href="https://www.avax.network/"> { "Avalanche" } </a> { ",
                which is already used to run Ethereum-compatible smart contracts.
                Currently, AT2 doesn't allow it but " }
                <a href="https://factory.c4dt.org/showcase/carbon">
                    { "Carbon" }
                </a> { ", the next iteration of AT2, aims to support it, with
                many added features, such as asset creation and a dynamic network.
            " } </p>

            <p> { "
                If you are interested in exploring this technology more, look at " }
                <a href="https://factory.c4dt.org/showcase/at2">
                    { "the project page" }
                </a> { "
                and contact the "
                } <a href="mailto:factory@c4dt.org"> { "C4DT" } </a> { ".
                We will be pleased to discuss it with you further."
            } </p>
        </> }
    }
}
