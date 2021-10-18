use std::{borrow::Cow, collections::HashMap, fmt, mem};

use at2_ns::ThinUser;
use js_sys::{JsString, Reflect};
use material_yew::{
    dialog::{ActionType, MatDialogAction},
    MatButton, MatDialog, MatFormfield, MatList, MatListItem, WeakComponentLink,
};
use yew::{prelude::*, services::ConsoleService, worker::Agent};

use crate::users_agent::UsersAgent;

const DEFAULT_SEND_TRANSACTION_AMOUNT: usize = 12;

// TODO snafu?
#[derive(Debug)]
struct MissingField;

impl fmt::Display for MissingField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("missing field")
    }
}
impl std::error::Error for MissingField {}

// TODO split
pub struct SendTransactionBuilder {
    amount: usize,
    user: Option<ThinUser>,
}

impl Default for SendTransactionBuilder {
    fn default() -> Self {
        Self {
            amount: DEFAULT_SEND_TRANSACTION_AMOUNT,
            user: None,
        }
    }
}

impl SendTransactionBuilder {
    fn set_amount(&mut self, amount: usize) {
        self.amount = amount;
    }
    fn set_user(&mut self, user: ThinUser) {
        self.user = Some(user);
    }
    fn build(self) -> Result<(ThinUser, usize), MissingField> {
        match self.user {
            Some(user) => Ok((user, self.amount)),
            None => Err(MissingField),
        }
    }
}

pub struct YourAccount {
    link: ComponentLink<Self>,

    #[allow(dead_code)] // never dropped
    users_agent: Box<dyn Bridge<UsersAgent>>,

    users: HashMap<String, ThinUser>,

    send_transaction_dialog: WeakComponentLink<MatDialog>,
    send_transaction_builder: SendTransactionBuilder,
}

pub enum Message {
    UsersAgent(<UsersAgent as Agent>::Output),

    ClickUser(String),
    UpdateAmountToSend(usize),
    SendTransaction,
    CancelSendTransaction,
}

impl Component for YourAccount {
    type Properties = ();
    type Message = Message;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            users_agent: UsersAgent::bridge(link.callback(Self::Message::UsersAgent)),
            link,
            users: HashMap::new(),
            send_transaction_dialog: Default::default(),
            send_transaction_builder: Default::default(),
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
            Self::Message::ClickUser(ref username) => {
                let user = self.users.get(username).unwrap().to_owned();
                self.send_transaction_builder.set_user(user);
                self.send_transaction_dialog.show();
                false
            }
            Self::Message::UpdateAmountToSend(amount) => {
                self.send_transaction_builder.set_amount(amount);
                false
            }
            Self::Message::SendTransaction => {
                let builder = mem::take(&mut self.send_transaction_builder);
                let (user, amount) = builder.build().unwrap();
                ConsoleService::info(&format!("send transaction: {} to {:?}", amount, user));
                false
            }
            Self::Message::CancelSendTransaction => {
                self.send_transaction_builder = SendTransactionBuilder::default();
                false
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
                            <MatListItem> { "Balance: ¤" } </MatListItem>
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
