use std::collections::HashMap;

use at2_node::{FullTransaction, TransactionState};
use at2_ns::{Contact, User};
use chrono::Utc;
use chrono_humanize::HumanTime;
use drop::crypto::sign;
use gloo_timers::callback::Interval;
use material_yew::{MatButton, MatFormfield};
use yew::{prelude::*, worker::Agent};

use super::select_user::SelectUser;
use crate::agents;

#[derive(Properties, Clone)]
pub struct Properties {
    /// User's account
    pub user: (User, sieve::Sequence),
    /// Where to send the new sequence when the current one is used
    pub bump_sequence: Callback<sieve::Sequence>,
}

pub struct YourAccount {
    link: ComponentLink<Self>,

    props: Properties,

    get_balance_agent: Box<dyn Bridge<agents::GetBalance>>,
    user_balance: Option<u64>,

    send_asset_agent: Box<dyn Bridge<agents::SendAsset>>,
    user_to_send_to: Option<Contact>,
    amount_to_send: String,

    #[allow(dead_code)] // never dropped
    get_latest_transactions_agent: Box<dyn Bridge<agents::GetLatestTransactions>>,
    latest_transactions: Vec<FullTransaction>,
    #[allow(dead_code)] // never dropped
    get_users_agent: Box<dyn Bridge<agents::GetUsers>>,
    pubkey_to_username: HashMap<sign::PublicKey, String>,
    #[allow(dead_code)] // never dropped
    refresher: Interval,
}

pub enum Message {
    GotBalance(<agents::GetBalance as Agent>::Output),

    UpdateAmount(String),
    SelectUser(Contact),
    SendTransaction,
    TransactionSent(<agents::SendAsset as Agent>::Output),

    LatestTransactionsGot(<agents::GetLatestTransactions as Agent>::Output),
    GotUsers(<agents::GetUsers as Agent>::Output),
    Refresh,
}

fn validate_amount(max: &Option<u64>, amount: &str) -> Option<u64> {
    let amount = amount.parse().ok()?;

    if let Some(max) = max {
        if amount > *max {
            return None;
        }
    }

    Some(amount)
}

impl Component for YourAccount {
    type Properties = Properties;
    type Message = Message;

    fn create(props: Properties, link: ComponentLink<Self>) -> Self {
        let send_asset_agent = agents::SendAsset::bridge(link.callback(Message::TransactionSent));
        let get_latest_transactions_agent =
            agents::GetLatestTransactions::bridge(link.callback(Message::LatestTransactionsGot));
        let get_users_agent = agents::GetUsers::bridge(link.callback(Message::GotUsers));

        let mut get_balance_agent = agents::GetBalance::bridge(link.callback(Message::GotBalance));
        get_balance_agent.send(props.user.0.clone().to_thin());

        let refresh = link.callback(|_: ()| Message::Refresh);

        Self {
            link,
            props,

            get_balance_agent,
            user_balance: None,

            send_asset_agent,
            user_to_send_to: None,
            amount_to_send: "3".to_owned(),

            get_latest_transactions_agent,
            latest_transactions: Vec::new(),
            get_users_agent,
            pubkey_to_username: HashMap::new(),
            refresher: Interval::new(1000, move || {
                refresh.emit(());
            }),
        }
    }

    fn update(&mut self, message: Message) -> ShouldRender {
        match message {
            Message::GotBalance(res) => {
                match res {
                    Ok(balance) => self.user_balance = Some(balance),
                    Err(_) => self
                        .get_balance_agent
                        .send(self.props.user.0.clone().to_thin()),
                }
                true
            }

            Message::SelectUser(user) => {
                self.user_to_send_to = Some(user);
                false
            }
            Message::UpdateAmount(amount) => {
                self.amount_to_send = amount;
                true
            }
            Message::SendTransaction => {
                if let Some(amount) = validate_amount(&self.user_balance, &self.amount_to_send) {
                    if let Some(user_to_send_to) = self.user_to_send_to.clone() {
                        let sequence = self.props.user.1 + 1;

                        self.send_asset_agent.send((
                            self.props.user.0.clone(),
                            sequence,
                            user_to_send_to,
                            amount,
                        ));

                        self.props.bump_sequence.emit(sequence);
                    }
                }

                // TODO in background
                self.get_balance_agent
                    .send(self.props.user.0.clone().to_thin());

                false
            }
            Message::TransactionSent(ret) => {
                ret.unwrap(); // TODO send asset in dialog
                false
            }

            Message::LatestTransactionsGot(mut latest_transactions) => {
                latest_transactions.reverse();
                self.latest_transactions = latest_transactions;

                self.get_balance_agent
                    .send(self.props.user.0.clone().to_thin());

                true
            }
            Message::GotUsers(users) => {
                self.pubkey_to_username = users
                    .into_iter()
                    .map(|user| (*user.public_key(), user.name.clone()))
                    .collect();

                true
            }

            Message::Refresh => true,
        }
    }

    fn change(&mut self, props: Properties) -> ShouldRender {
        self.props = props;
        false
    }

    fn view(&self) -> Html {
        let now = Utc::now();

        html! { <>
            <h1> { "Your account" } </h1>

            <p> { "
                Now, your account is registered on the chain.
                Same as Bitcoin, you have a wallet, to which we added 100'000
                assets.
            " } <br /> { "
                Below, you can play by sending some asset to the other accounts
                of the network. Click on any name, send some asset and see your
                transaction being validated.
            " } <br /> { "
                The ten most recent transactions on the network will appear
                below, with the most recent on top.
                If you see it changing rapidly, that's probably because
                someone else is running a speedtest.
            " } <br /> { "
                If you don't see your transactions, or that one fails, please " }
                <a href="mailto:factory@c4dt.org"> { "contact the C4DT" } </a>
                { "." }
            </p>

            <hr />

            <h2> { "Send assets" } </h2>

            <span style=concat!(
                "display: flex;",
                "flex-direction: column;",
                "justify-content: space-around;",
            )>
                <p>
                    { "Your balance: " }
                    { self.user_balance
                        .map(|balance| html! { format!("{} ₳", balance) })
                        .unwrap_or(html! { <span style="color: lightgrey"> { "fetching" } </span> }) }
                </p>

                <span>
                    <label>
                        { "Send " }
                        <input
                            oninput=self.link.callback(|event: InputData|
                                Message::UpdateAmount(event.value))
                            value=self.amount_to_send.clone()
                            min=1
                            type={ "number" }
                        />
                    </label>

                    { " ₳ " }

                    <MatFormfield label="to" align_end=true>
                        <SelectUser
                            user_selected=self.link.callback(Message::SelectUser)
                        />
                    </MatFormfield>
                </span>

                <span
                    onclick=self.link.callback(|_| Message::SendTransaction)
                    style="padding: 1em 0"
                ><MatButton
                    label="Send"
                    raised=true
                    disabled=validate_amount(&self.user_balance, &self.amount_to_send).is_none()
                /></span>
            </span>

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
                      <td style="padding: 0 2px;">{ tx.amount } { " ₳" }</td>
                  </tr>
                }) }
            </table>

        </> }
    }
}
