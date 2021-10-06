use at2_ns::{client::Client, ThinUser};
use gloo_timers::callback::Interval;
use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, services::ConsoleService};

use crate::config::Config;

pub struct YourAccount {
    users: Vec<ThinUser>,
}

pub enum Message {
    NewUsers(Vec<ThinUser>),
}

impl Component for YourAccount {
    type Properties = ();
    type Message = Message;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let conf = Config::parse().unwrap(); // TODO unwrap

        Interval::new(1_000, move || {
            let mut client = Client::new(conf.name_service());
            let send_new_users = link.callback(Self::Message::NewUsers);

            spawn_local(async move {
                match client.get_all().await {
                    Ok(users) => send_new_users.emit(users),
                    Err(err) => ConsoleService::error(&format!("unable to refresh users: {}", err)),
                }
            });
        })
        .forget();

        Self { users: Vec::new() }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Self::Message::NewUsers(users) => {
                self.users = users;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        struct Transaction<'a> {
            started: &'a str,
            from: &'a str,
            to: &'a str,
            state: &'a str,
            amount: usize,
        }

        let transactions = vec![
            Transaction {
                started: "3s ago",
                from: "Marie",
                to: "Brigitte",
                state: "Received, waiting for confirmation",
                amount: 20,
            },
            Transaction {
                started: "20min ago",
                from: "tharvik",
                to: "Alice",
                state: "Confirmed",
                amount: 6,
            },
            Transaction {
                started: "3 days ago",
                from: "Alice",
                to: "Marie",
                state: "Confirmed",
                amount: 565,
            },
        ];

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

            <span class=classes!("boxes")>
                { for self.users.iter().map(|user| html! {
                    <p> { user.name() } </p>
                }) }
            </span>

            <hr />

            <h2> { "Transactions" } </h2>

            <table style=concat!(
                "width: 100%;",
                "border-collapse: collapse;",
            )>
                { for transactions.iter().map(|tx| html! {
                  <tr style=concat!(
                      "border-bottom: 1px solid;",
                      "border-top: 1px solid;",
                  )>
                      <td>{ tx.started }</td>
                      <td>{ tx.from } { " -> " } { tx.to }</td>
                      <td>{ tx.state }</td>
                      <td>{ tx.amount } { "¤" }</td>
                  </tr>
                }) }
            </table>

        </div> }
    }
}
