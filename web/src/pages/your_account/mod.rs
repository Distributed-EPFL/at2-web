use std::collections::HashMap;

use at2_ns::{FullUser, ThinUser};
use js_sys::{JsString, Reflect};
use material_yew::{MatButton, MatDialog, WeakComponentLink};
use yew::{prelude::*, worker::Agent};

use crate::agents;

mod send_transaction_dialog;
use send_transaction_dialog::SendTransactionDialog;
mod transaction_builder;

#[derive(Properties, Clone)]
pub struct Properties {
    pub user: (FullUser, sieve::Sequence),
    pub bump_sequence: Callback<sieve::Sequence>,
}

pub struct YourAccount {
    link: ComponentLink<Self>,

    props: Properties,

    #[allow(dead_code)] // never dropped
    get_users_agent: Box<dyn Bridge<agents::GetUsers>>,
    send_asset_agent: Box<dyn Bridge<agents::SendAsset>>,

    users: HashMap<String, ThinUser>,

    dialog_link: WeakComponentLink<MatDialog>,
    dialog_user: Option<ThinUser>,
}

pub enum Message {
    GotUsers(<agents::GetUsers as Agent>::Output),
    AssetSent(<agents::SendAsset as Agent>::Output),

    ClickUser(String),
    SendTransaction((ThinUser, usize)),
}

impl Component for YourAccount {
    type Properties = Properties;
    type Message = Message;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: link.clone(),
            props,
            get_users_agent: agents::GetUsers::bridge(link.callback(Self::Message::GotUsers)),
            send_asset_agent: agents::SendAsset::bridge(link.callback(Self::Message::AssetSent)),
            users: HashMap::new(),
            dialog_link: Default::default(),
            dialog_user: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Self::Message::GotUsers(users) => {
                self.users = users.into_iter().map(|user| (user.name.clone(), user)).collect();
                true
            }
            Self::Message::ClickUser(ref username) => {
                let user = self.users.get(username).unwrap().to_owned();

                self.dialog_user = Some(user);
                self.dialog_link.show();
                false
            }
            Self::Message::SendTransaction((recipient, amount)) => {
                let sequence = self.props.user.1 + 1;

                self.send_asset_agent.send((
                    self.props.user.0.clone(),
                    sequence,
                    recipient,
                    amount as u64,
                ));

                self.props.bump_sequence.emit(sequence);

                false
            }
            Self::Message::AssetSent(ret) => {
                ret.unwrap(); // TODO unwrap
                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let mut users = self.users.values().cloned().collect::<Vec<_>>();
        users.sort_by(|l, r| l.name.cmp(&r.name));

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
                { for users.into_iter().map(|user| html! {
                    <span
                        onclick=self.link.callback(|event: MouseEvent|
                            Self::Message::ClickUser(
                                Reflect::get(
                                    event.target().unwrap().as_ref(),
                                    &JsString::from("label"),
                                )
                                .unwrap()
                                .as_string()
                                .unwrap()
                            )
                        )
                    ><MatButton
                        label=user.name
                        raised=true
                    /></span>
                } ) }
            </span>

            <SendTransactionDialog
                dialog_link=self.dialog_link.clone()
                user=self.dialog_user.clone()
                on_send=self.link.callback(Self::Message::SendTransaction)
            />

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
