use yew::prelude::*;

pub struct YourAccount {}

impl Component for YourAccount {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! { <div class=classes!("page")>
            <h1> { "Your account" } </h1>

            <p>
                { "Now, you're registered on the chain." }
                <br />
                { "As with BitCoin, you have a wallet, already populated." }
                <br />
                { "Below, you can play by sending some money to the other
                  members of the network. Click on any name, send some money
                  and see your transaction being validated." }
            </p>

            <hr />

            <h2> { "Addressbook" } </h2>

        </div> }
    }
}
