use std::{borrow::Cow, collections::HashMap, mem};

use at2_ns::{FullUser, ThinUser};
use js_sys::{JsString, Reflect};
use material_yew::{
    dialog::{ActionType, MatDialogAction},
    MatButton, MatDialog, MatFormfield, MatList, MatListItem, WeakComponentLink,
};
use yew::{prelude::*, worker::Agent};

use crate::agents;

mod send_transaction_builder;
use send_transaction_builder::{SendTransactionBuilder, DEFAULT_SEND_TRANSACTION_AMOUNT};

#[derive(Properties, Clone)]
pub struct Properties {
    pub user: FullUser,
}

pub struct YourAccount {
    link: ComponentLink<Self>,

    props: Properties,

    #[allow(dead_code)] // never dropped
    get_users_agent: Box<dyn Bridge<agents::GetUsers>>,
    get_balance_agent: Box<dyn Bridge<agents::GetBalance>>,
    send_asset_agent: Box<dyn Bridge<agents::SendAsset>>,

    users: HashMap<String, ThinUser>,
    current_user_balance: Option<u64>,

    send_transaction_dialog: WeakComponentLink<MatDialog>,
    send_transaction_builder: SendTransactionBuilder,
}

pub enum Message {
    GotUsers(<agents::GetUsers as Agent>::Output),
    GotBalance(<agents::GetBalance as Agent>::Output),
    AssetSent(<agents::SendAsset as Agent>::Output),

    ClickUser(String),
    UpdateAmountToSend(usize),
    SendTransaction,
    CancelSendTransaction,
}

impl Component for YourAccount {
    type Properties = Properties;
    type Message = Message;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: link.clone(),
            props,
            get_users_agent: agents::GetUsers::bridge(link.callback(Self::Message::GotUsers)),
            get_balance_agent: agents::GetBalance::bridge(link.callback(Self::Message::GotBalance)),
            send_asset_agent: agents::SendAsset::bridge(link.callback(Self::Message::AssetSent)),
            users: HashMap::new(),
            current_user_balance: None,
            send_transaction_dialog: Default::default(),
            send_transaction_builder: Default::default(),
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Self::Message::GotUsers(users) => {
                self.users = users
                    .into_iter()
                    .map(|user| (user.name().to_owned(), user))
                    .collect();
                true
            }
            Self::Message::ClickUser(ref username) => {
                let user = self.users.get(username).unwrap().to_owned();
                self.get_balance_agent.send(user.clone());
                self.send_transaction_builder.set_user(user);
                self.send_transaction_dialog.show();
                false
            }
            Self::Message::UpdateAmountToSend(amount) => {
                self.send_transaction_builder.set_amount(amount);
                false
            }
            Self::Message::SendTransaction => {
                self.current_user_balance = None;

                let builder = mem::take(&mut self.send_transaction_builder);
                let user = self.props.user.clone();
                let (recipient, amount) = builder.build().unwrap();
                let sequence = 1; // TODO retrieve latest sequence

                self.send_asset_agent
                    .send((user, sequence, recipient, amount as u64));

                false
            }
            Self::Message::CancelSendTransaction => {
                self.current_user_balance = None;

                self.send_transaction_builder = SendTransactionBuilder::default();
                false
            }
            Self::Message::AssetSent(ret) => {
                ret.unwrap(); // TODO unwrap
                false
            }
            Self::Message::GotBalance(ret) => {
                self.current_user_balance = Some(ret.unwrap()); // TODO unwrap
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let mut users = self.users.values().cloned().collect::<Vec<_>>();
        users.sort_by(|l, r| l.name().cmp(r.name()));

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
                { for users.into_iter().map(|user| html! { <>
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
                        label=user.name().to_owned()
                        raised=true
                    /></span>

                    <MatDialog
                        heading=Cow::from(user.name().to_owned())
                        dialog_link=self.send_transaction_dialog.clone()
                        onclosed=self.link.callback(|action: String| match action.as_str() {
                            "send" => Self::Message::SendTransaction,
                            "cancel" => Self::Message::CancelSendTransaction,
                            _ => unreachable!(),
                        })
                    >
                        <MatList >
                            <MatListItem>
                                { "Balance: " }
                                { self.current_user_balance
                                    .map(|balance| html! { format!("{}¤", balance) })
                                    .unwrap_or(html! { <span style="color: lightgrey"> { "fetching" } </span> }) }
                            </MatListItem>
                            <MatListItem> { format!("Public key: {}", user.public_key()) } </MatListItem>
                            <MatListItem>
                                <MatFormfield
                                    label="Amount to send"
                                    align_end=true
                                ><input
                                    value=DEFAULT_SEND_TRANSACTION_AMOUNT.to_string()
                                    oninput=self.link.callback(|event: InputData|
                                        Self::Message::UpdateAmountToSend(event.value.parse().unwrap())
                                    )
                                    type="number"
                                /></MatFormfield>
                            </MatListItem>
                        </MatList>

                        <MatDialogAction
                            action_type=ActionType::Primary
                            action=Cow::from("send")>
                            <MatButton label="Send" />
                        </MatDialogAction>
                        <MatDialogAction
                            action_type=ActionType::Secondary
                            action=Cow::from("cancel")>
                            <MatButton label="Cancel" />
                        </MatDialogAction>
                    </MatDialog>
                </> }) }
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
