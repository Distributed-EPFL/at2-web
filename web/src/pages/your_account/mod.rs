use std::{collections::HashMap, convert::TryInto};

use at2_node::{FullTransaction, TransactionState};
use at2_ns::{Contact, User};
use chrono::Utc;
use chrono_humanize::HumanTime;
use drop::crypto::sign;
use gloo_console::error;
use js_sys::{JsString, Reflect};
use material_yew::{MatButton, MatDialog, WeakComponentLink};
use yew::prelude::*;
use yew_agent::{Agent, Bridge};

use crate::agents;

mod send_transaction_dialog;
use send_transaction_dialog::SendTransactionDialog;

#[derive(Properties, Clone, PartialEq)]
pub struct Properties {
    /// User's account
    pub user: (User, sieve::Sequence),
    /// Where to send the new sequence when the current one is used
    pub bump_sequence: Callback<sieve::Sequence>,
}

pub struct YourAccount {
    props: Properties,

    #[allow(dead_code)] // never dropped
    get_users_agent: Box<dyn Bridge<agents::GetUsers>>,
    send_asset_agent: Box<dyn Bridge<agents::SendAsset>>,
    #[allow(dead_code)] // never dropped
    get_latest_transactions_agent: Box<dyn Bridge<agents::GetLatestTransactions>>,

    sorted_usernames: Vec<String>,
    username_to_user: HashMap<String, Contact>,
    pubkey_to_username: HashMap<sign::PublicKey, String>,

    dialog_link: WeakComponentLink<MatDialog>,
    dialog_user: Option<Contact>,

    latest_transactions: Vec<FullTransaction>,
}

pub enum Message {
    GotUsers(<agents::GetUsers as Agent>::Output),
    TransactionSent(<agents::SendAsset as Agent>::Output),
    LatestTransactionsGot(<agents::GetLatestTransactions as Agent>::Output),

    ClickUser(Option<String>),
    SendTransaction((Contact, usize)),
}

impl Component for YourAccount {
    type Properties = Properties;
    type Message = Message;

    fn create(ctx: Context<Self>) -> Self {
        let get_users_agent =
            agents::GetUsers::bridge(ctx.link().callback(Self::Message::GotUsers));
        let send_asset_agent =
            agents::SendAsset::bridge(ctx.link().callback(Self::Message::TransactionSent));
        let get_latest_transactions_agent = agents::GetLatestTransactions::bridge(
            ctx.link().callback(Self::Message::LatestTransactionsGot),
        );

        Self {
            props: ctx.props(),

            get_users_agent,
            send_asset_agent,
            get_latest_transactions_agent,
            sorted_usernames: Vec::new(),
            username_to_user: HashMap::new(),
            pubkey_to_username: HashMap::new(),

            dialog_link: Default::default(),
            dialog_user: None,

            latest_transactions: Vec::new(),
        }
    }

    fn update(&mut self, message: Self::Message) -> bool {
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
                    error!("unable to fit in u64:", amount);
                }

                false
            }
            Self::Message::TransactionSent(ret) => {
                ret.unwrap(); // TODO send asset in dialog
                false
            }
            Self::Message::LatestTransactionsGot(mut latest_transactions) => {
                latest_transactions.reverse();
                self.latest_transactions = latest_transactions;
                true
            }
        }
    }

    fn changed(&mut self, ctx: Context<Self>) -> bool {
        self.props = ctx.props();
        false
    }

    fn view(&self) -> Html {
        let now = Utc::now();

        html! { <>
            <h1> { "Your account" } </h1>

            <p> { "
                Now, your account is registered on the chain.
                As with BitCoin, you have a wallet, which is already populated.
            " } <br /> { "
                Below, you can play by sending some asset to the other members
                of the network. Click on any name, send some asset and see your
                transaction being validated.
            " } <br /> { "
                The most recent transactions will appear below, with the most
                recent on top.
                If you see it changing rapidly, that's probably because
                someone else is running a speedtest.
            " } </p>

            <p> { "
                One of the specificity of AT2 is that there isn't a regular
                synchronisation of the network.
                So when one is sending a transaction that debits more than one
                have, we can't really say it's invalid. There might be
                another transaction crediting this account that is still
                spreading throughout the network. So it will stay in a pending
                state until the network remove it.
            " } <br /> { "
                But sending wrong transactions is punitive here, if you send
                one, the network will kick you out. In the protocol itself, it
                will be seen as trying to double spend.
                Please don't, or wait a minute before retrying.
            " } </p>

            <hr />

            <h2> { "Addressbook" } </h2>

            <span style={ concat!(
                "display: flex;",
                "flex-wrap: wrap;",
                "align-items: center;",
                "justify-content: space-around;",
            ) }>
                { for self.sorted_usernames.iter().cloned().map(|username| html! {
                    <span
                        onclick={ self.link.callback(|event: MouseEvent|
                            Self::Message::ClickUser((|| {
                                Reflect::get(
                                    event.target()?.as_ref(),
                                    &JsString::from("label"),
                                )
                                .ok()?
                                .as_string()
                            })())
                        ) }
                    ><MatButton
                        label={ username }
                        raised=true
                    /></span>
                } ) }
            </span>

            <SendTransactionDialog
                dialog_link={ self.dialog_link.clone() }
                user={ self.dialog_user.clone() }
                on_send={ self.link.callback(Self::Message::SendTransaction) }
            />

            <hr />

            <h2> { "Transactions" } </h2>

            <table style="width: 100%; border-collapse: collapse;">
                { for self.latest_transactions.iter().map(|tx| html! {
                  <tr style="border-bottom: 1px solid; border-top: 1px solid;">
                      <td style="padding: 0 2px;">{ HumanTime::from(tx.timestamp - now) }</td>
                      <td style="padding: 0 2px;">{ match tx.state {
                          TransactionState::Pending => html! { <span style="color: grey">{ "pending" }</span> },
                          TransactionState::Success => html! { "success" },
                          TransactionState::Failure => html! { <span style="color: violet">{ "failure" }</span> },
                      }}</td>
                      <td style="padding: 0 2px;">
                        { self.pubkey_to_username.get(&tx.sender).unwrap_or(&tx.sender.to_string()) }
                        { " -> " }
                        { self.pubkey_to_username.get(&tx.recipient).unwrap_or(&tx.recipient.to_string()) }</td>
                      <td style="padding: 0 2px;">{ tx.amount } { "💶" }</td>
                  </tr>
                }) }
            </table>

        </> }
    }
}
