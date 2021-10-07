use yew::prelude::*;

pub struct Speedtest {
    link: ComponentLink<Self>,

    is_running: bool,
    amount: usize,
    to_user: Option<String>,
}

pub enum Message {
    UpdateTransactionAmount(usize),
    UpdateUser(String),
    Start,
}

impl Component for Speedtest {
    type Properties = ();
    type Message = Message;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            is_running: false,
            amount: 0,
            to_user: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Self::Message::Start => {
                self.is_running = true;
                true
            }

            Self::Message::UpdateTransactionAmount(amount) => {
                self.amount = amount;
                false
            }
            Self::Message::UpdateUser(username) => {
                self.to_user = Some(username);
                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // TODO fetch and store users in agent
        let users = ["Alice", "Betty", "Catherine"];

        html! { <div class=classes!("page")>
            <h1> { "Speedtest" } </h1>

            <p>
                { "One of the most interesting feature of AT2 is that it allows
                   for many more transactions per second (TPS) than currently
                   existing blockchains. Here you can actually test it,
                   flooding the servers with transactions." }
            </p>

            <hr />

            <div style=concat!(
                "display: flex;",
                "flex-direction: column;",
                "align-items: center;",
            )>
                <label> { "How many transactions to send" }
                    <input
                        oninput=self.link.callback(|event: InputData|
                            Self::Message::UpdateTransactionAmount(event.value.parse().unwrap())) // TODO unwrap
                        type={ "number" }
                    />
                 </label>

                <label>
                    { "To whom to send to" }
                    <select
                        // TODO use oninput directly?
                        onchange=self.link.callback(|event: ChangeData| match event {
                            ChangeData::Select(elem) => Self::Message::UpdateUser(elem.value()),
                            _ => unreachable!(),
                        })
                    >
                        { for ["Anyone"].iter().chain(&users).map(|user| html! { <option>{ user }</option> }) }
                    </select>
                 </label>

                <button
                    onclick=self.link.callback(|_| Self::Message::Start)
                    disabled=self.is_running
                > { "Launch" } </button>
            </div>

            <hr />

            <div hidden=!self.is_running>
                <div style=concat!(
                    "display: flex;",
                    "flex-direction: column;",
                )>
                    { "Transactions sent: 17/343" }
                    <br />
                    { "Transactions confirmed: 14/17" }
                </div>

                <p> { "Running for 0.1s" } </p>

                <div style=concat!(
                    "display: flex;",
                    "flex-direction: column;",
                )>
                    { "AT2's TPS: 170" }
                    <br />
                    { "Bitcoin's TPS: 5" } // TODO check values
                    <br />
                    { "Ethereum's TPS: 16" }
                </div>
            </div>

        </div> }
    }
}
