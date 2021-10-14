use std::collections::HashMap;

use at2_ns::ThinUser;
use js_sys::{JsString, Reflect};
use material_yew::MatButton;
use yew::{prelude::*, services::ConsoleService, worker::Agent};

use crate::users_agent::UsersAgent;

pub struct YourAccount {
    link: ComponentLink<Self>,

    #[allow(dead_code)] // never dropped
    users_agent: Box<dyn Bridge<UsersAgent>>,

    users: HashMap<String, ThinUser>,
}

pub enum Message {
    UsersAgent(<UsersAgent as Agent>::Output),
    ClickUser(String),
}

impl Component for YourAccount {
    type Properties = ();
    type Message = Message;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            users_agent: UsersAgent::bridge(link.callback(Self::Message::UsersAgent)),
            link,
            users: HashMap::new(),
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Self::Message::UsersAgent(users) => {
                self.users = users
                    .into_iter()
                    .map(|user| (user.name().to_owned(), user))
                    .collect();
                true
            }
            Self::Message::ClickUser(username) => {
                let user = self.users.get(&username);
                ConsoleService::info(&format!("click user: {:?}", user));
                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let mut usernames = self.users.keys().cloned().collect::<Vec<_>>();
        usernames.sort();

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

        html! { <>
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
                { for usernames.into_iter().map(|username| html! {
                    <span
                        onclick=self.link.callback(|event: MouseEvent|
                            Self::Message::ClickUser(
                                Reflect::get(
                                    event.target().unwrap().as_ref(),
                                    &JsString::from("textContent"),
                                )
                                .unwrap()
                                .as_string()
                                .unwrap()
                            )
                        )
                    ><MatButton
                        label=username
                        raised=true
                    /></span>
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

        </> }
    }
}
