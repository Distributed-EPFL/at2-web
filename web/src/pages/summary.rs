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

            <p> { "
                We showed that AT2 is behaving as any other blockchain.
                You can send asset with it, see other users on the network
                and see the latest processed transactions.
                But using a speedtest really uncovers the true power of AT2:
                it is faster than other most popular blockchains, beating
                Bitcoin and Etherium by a factor of 10.
                It achieves that speed by avoiding to sync a common state,
                the global consensus. It uses a local one, allowing nodes
                to validate transactions before checking them with every other
                node on the network.
            " } </p>

            <p> { "
                As the transaction time is so short and because it can run in a
                browser, one can imagine using it directly as a means of
                payment, having a little button \"pay with AT2\" next to the
                credit cards.
                Or you can use AT2 directly as a means of communication, an
                instant chat ensuring that no rogue participant can interfere
                or reorder the messages.
            " } </p>

            <p> { "
                Most blockchains are still using the global consensus method,
                but AT2's paper demonstrates that this it is unneeded to
                transfer asset inside a network.
                For the sake of transparency, there is another project based
                on a similar technology, though not on the same papers, " }
                <a href="https://www.avax.network/"> { "Avalanche" } </a> { ",
                which is already used to run Etherium-compatible smart contracts.
                Currently, AT2 doesn't allow it but " }
                <a href="https://factory.c4dt.org/showcase/carbon">
                    { "Carbon" }
                </a> { ", the next iteration of AT2, aims to support it, with
                many added features, such as minting and a dynamic network.
            " } </p>

            <p> { "
                If you are interested in exploring this technology more, look at " }
                <a href="https://factory.c4dt.org/showcase/at2">
                    { "the project page" }
                </a> { "
                and contact the C4DT, we will be pleased to discuss it with you
                in greater details."
            } </p>
        </> }
    }
}
