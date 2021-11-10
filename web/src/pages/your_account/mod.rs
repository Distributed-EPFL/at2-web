use std::{collections::HashMap, convert::TryInto};

use at2_node::FullTransaction;
use at2_ns::{FullUser, ThinUser};
use chrono::Utc;
use chrono_humanize::HumanTime;
use drop::crypto::sign;
use js_sys::{JsString, Reflect};
use material_yew::{MatButton, MatDialog, WeakComponentLink};
use yew::{prelude::*, services::ConsoleService, worker::Agent};

use crate::agents;

mod send_transaction_dialog;
use send_transaction_dialog::SendTransactionDialog;

#[derive(Properties, Clone)]
pub struct Properties {
    /// User's account
    pub user: (FullUser, sieve::Sequence),
    /// Where to send the new sequence when the current one is used
    pub bump_sequence: Callback<sieve::Sequence>,
}

pub struct YourAccount {
    link: ComponentLink<Self>,

    props: Properties,

    #[allow(dead_code)] // never dropped
    get_users_agent: Box<dyn Bridge<agents::GetUsers>>,
    send_asset_agent: Box<dyn Bridge<agents::SendAsset>>,
    #[allow(dead_code)] // never dropped
    get_latest_transactions_agent: Box<dyn Bridge<agents::GetLatestTransactions>>,

    sorted_usernames: Vec<String>,
    username_to_user: HashMap<String, ThinUser>,
    pubkey_to_username: HashMap<sign::PublicKey, String>,

    dialog_link: WeakComponentLink<MatDialog>,
    dialog_user: Option<ThinUser>,

    latest_transactions: Vec<FullTransaction>,
}

pub enum Message {
    GotUsers(<agents::GetUsers as Agent>::Output),
    AssetSent(<agents::SendAsset as Agent>::Output),
    LatestTransactionsGot(<agents::GetLatestTransactions as Agent>::Output),

    ClickUser(Option<String>),
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
            get_latest_transactions_agent: agents::GetLatestTransactions::bridge(
                link.callback(Self::Message::LatestTransactionsGot),
            ),

            sorted_usernames: Vec::new(),
            username_to_user: HashMap::new(),
            pubkey_to_username: HashMap::new(),

            dialog_link: Default::default(),
            dialog_user: None,

            latest_transactions: Vec::new(),
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Self::Message::GotUsers(users) => {
                let mut sorted_usernames = users
                    .iter()
                    .map(|user| user.name.clone())
                    .collect::<Vec<_>>();
                sorted_usernames.sort_unstable();
                self.sorted_usernames = sorted_usernames;

                self.username_to_user = users
                    .iter()
                    .cloned()
                    .map(|user| (user.name.clone(), user))
                    .collect();

                self.pubkey_to_username = users
                    .into_iter()
                    .map(|user| (*user.public_key(), user.name.clone()))
                    .collect();

                true
            }
            Self::Message::ClickUser(found_username) => {
                if let Some(user) = found_username
                    .as_ref()
                    .and_then(|username| self.username_to_user.get(username))
                {
                    self.dialog_user = Some(user.to_owned());
                    self.dialog_link.show();
                }
                true
            }
            Self::Message::SendTransaction((recipient, amount)) => {
                if let Ok(amount) = amount.try_into() {
                    let sequence = self.props.user.1 + 1;

                    self.send_asset_agent.send((
                        self.props.user.0.clone(),
                        sequence,
                        recipient,
                        amount,
                    ));

                    self.props.bump_sequence.emit(sequence);
                } else {
                    ConsoleService::error(&format!("unable to fit {} in u64", amount));
                }

                false
            }
            Self::Message::AssetSent(ret) => {
                ret.unwrap(); // TODO send asset in dialog
                false
            }
            Self::Message::LatestTransactionsGot(latest_transactions) => {
                self.latest_transactions = latest_transactions;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let now = Utc::now();

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
                { for self.sorted_usernames.iter().cloned().map(|username| html! {
                    <span
                        onclick=self.link.callback(|event: MouseEvent|
                            Self::Message::ClickUser((|| {
                                Reflect::get(
                                    event.target()?.as_ref(),
                                    &JsString::from("label"),
                                )
                                .ok()?
                                .as_string()
                            })())
                        )
                    ><MatButton
                        label=username
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
                { for self.latest_transactions.iter().map(|tx| html! {
                  <tr style=concat!(
                      "border-bottom: 1px solid;",
                      "border-top: 1px solid;",
                  )>
                      <td>{ HumanTime::from(tx.timestamp - now) }</td>
                      <td>
                        { self.pubkey_to_username.get(&tx.sender).unwrap_or(&tx.sender.to_string()) }
                        { " -> " }
                        { self.pubkey_to_username.get(&tx.recipient).unwrap_or(&tx.recipient.to_string()) }</td>
                      <td>{ tx.amount } { "¤" }</td>
                  </tr>
                }) }
            </table>

        </> }
    }
}
