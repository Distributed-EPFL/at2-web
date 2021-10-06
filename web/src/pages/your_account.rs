use std::time::Duration;

use at2_ns::{Client, ThinUser};
use wasm_bindgen_futures::spawn_local;
use yew::{
    prelude::*,
    services::{interval::IntervalTask, ConsoleService, IntervalService},
};

use super::super::config::NAME_SERVICE_URI;

pub struct YourAccount {
    send_new_users: Callback<Vec<ThinUser>>,
    users: Vec<ThinUser>,

    // no way to stop refreshing
    #[allow(dead_code)]
    fetch_users: IntervalTask,
}

pub enum Message {
    FetchUsers,
    NewUsers(Vec<ThinUser>),
}

impl Component for YourAccount {
    type Properties = ();
    type Message = Message;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            send_new_users: link.callback(Self::Message::NewUsers),
            users: Vec::new(),

            fetch_users: IntervalService::spawn(
                Duration::from_secs(1),
                link.callback(|_| Self::Message::FetchUsers),
            ),
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Self::Message::FetchUsers => {
                let callback = self.send_new_users.clone();

                spawn_local(async move {
                    let mut client = Client::new(NAME_SERVICE_URI.parse().unwrap()); // TODO unwrap
                    let users = client.get_all().await.unwrap(); // TODO unwrap

                    ConsoleService::info(&format!("users: {:?}", users));

                    callback.emit(users);
                });

                false
            }
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
