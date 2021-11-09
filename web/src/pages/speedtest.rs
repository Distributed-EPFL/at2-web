use std::collections::HashSet;

use at2_ns::ThinUser;
use yew::{prelude::*, worker::Agent};

use crate::agents;

pub struct Speedtest {
    link: ComponentLink<Self>,

    #[allow(dead_code)] // never dropped
    users_agent: Box<dyn Bridge<agents::GetUsers>>,
    users: HashSet<ThinUser>,

    is_running: bool,
    amount: usize,
    to_user: Option<String>,
}

pub enum Message {
    GotUsers(<agents::GetUsers as Agent>::Output),
    UpdateTransactionAmount(Option<usize>),
    UpdateUser(String),
    Start,
}

impl Component for Speedtest {
    type Properties = ();
    type Message = Message;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let users_agent = agents::GetUsers::bridge(link.callback(Self::Message::GotUsers));

        Self {
            link,
            users_agent,
            users: HashSet::new(),
            is_running: false,
            amount: 0,
            to_user: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Self::Message::GotUsers(users) => {
                self.users = users;
                true
            }
            Self::Message::Start => {
                self.is_running = true;
                true
            }
            Self::Message::UpdateTransactionAmount(amount) => {
                if let Some(amount) = amount {
                    self.amount = amount;
                }
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
        html! { <>
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
                            Self::Message::UpdateTransactionAmount(event.value.parse().ok()))
                        value=100
                        min=1
                        type={ "number" }
                    />
                 </label>

                <label>
                    { "To whom to send to" }
                    <select
                        onchange=self.link.callback(|event: ChangeData| match event {
                            ChangeData::Select(elem) => Self::Message::UpdateUser(elem.value()),
                            _ => unreachable!(),
                        })
                    >
                        <option>{ "Anyone" }</option>
                        { for self.users.iter().map(|user| html! {
                            <option>{ user.name.clone() }</option>
                        }) }
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

        </> }
    }
}
