use std::collections::HashMap;

use at2_ns::{FullUser, ThinUser};
use chrono::{offset::Local, DateTime, Duration};
use gloo_timers::callback::{Interval, Timeout};
use rand::{seq::SliceRandom, thread_rng};
use yew::{prelude::*, services::ConsoleService, worker::Agent};

use crate::agents;

const TRANSFER_PER_REFRESH: usize = 50;

#[derive(Properties, Clone)]
pub struct Properties {
    /// User's account
    pub user: (FullUser, sieve::Sequence),
    /// Where to send the new sequence when the current one is used
    pub bump_sequence: Callback<sieve::Sequence>,
}

pub struct Speedtest {
    link: ComponentLink<Self>,
    props: Properties,

    #[allow(dead_code)] // never dropped
    users_agent: Box<dyn Bridge<agents::GetUsers>>,
    send_asset_agent: Box<dyn Bridge<agents::SendAsset>>,

    sorted_usernames: Vec<String>,
    username_to_user: HashMap<String, ThinUser>,

    amount: usize,
    to_username: Option<String>,

    state: State,
    #[allow(dead_code)] // never dropped
    last_sequence_refresher: Interval,
}

pub enum State {
    Idle,
    Started {
        started_at: DateTime<Local>,

        sent_tx: usize,
        confirmed_tx: usize,
        total_tx: usize,
    },
    Done {
        elapsed: Duration,
        total_tx: usize,
    },
}

pub enum Message {
    TransactionSent(<agents::SendAsset as Agent>::Output),
    GotUsers(<agents::GetUsers as Agent>::Output),
    GotLastSequence(<agents::GetLastSequence as Agent>::Output),

    UpdateTransactionAmount(Option<usize>),
    UpdateUser(String),

    Start,
    Running {
        users_to_send_to: Vec<ThinUser>,
        remaining: usize,
    },
}

impl Component for Speedtest {
    type Properties = Properties;
    type Message = Message;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let users_agent = agents::GetUsers::bridge(link.callback(Self::Message::GotUsers));
        let send_asset_agent =
            agents::SendAsset::bridge(link.callback(Self::Message::TransactionSent));

        let user = props.user.0.clone().to_thin();
        let mut get_last_sequence_agent =
            agents::GetLastSequence::bridge(link.callback(Self::Message::GotLastSequence));

        Self {
            link,
            props,

            users_agent,
            send_asset_agent,

            sorted_usernames: Vec::new(),
            username_to_user: HashMap::new(),

            amount: 100,
            to_username: None,

            state: State::Idle,
            last_sequence_refresher: Interval::new(100, move || {
                get_last_sequence_agent.send(user.clone())
            }),
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Self::Message::TransactionSent(_) => {
                if let State::Started { sent_tx, .. } = &mut self.state {
                    *sent_tx += 1;

                    *sent_tx % TRANSFER_PER_REFRESH == 0
                } else {
                    false
                }
            }

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

                true
            }
            Self::Message::GotLastSequence(ret) => {
                if let State::Started {
                    started_at,
                    confirmed_tx,
                    total_tx,
                    ..
                } = &mut self.state
                {
                    if let Ok(seq) = ret {
                        ConsoleService::info(&format!(
                            "got last sequence: confirmed_tx={} seq={} base_seq={}",
                            confirmed_tx, seq, self.props.user.1
                        ));
                        *confirmed_tx = (seq - self.props.user.1) as usize;

                        if confirmed_tx == total_tx {
                            self.state = State::Done {
                                elapsed: Local::now() - *started_at,
                                total_tx: *total_tx,
                            };

                            self.props.bump_sequence.emit(seq);
                        }
                    }
                }
                true
            }

            Self::Message::UpdateTransactionAmount(amount) => {
                if let Some(amount) = amount {
                    self.amount = amount;
                }
                false
            }
            Self::Message::UpdateUser(username) => {
                self.to_username = Some(username);
                false
            }

            Self::Message::Start => {
                let total_tx = self.amount;

                self.state = State::Started {
                    started_at: Local::now(),
                    sent_tx: 0,
                    confirmed_tx: 0,
                    total_tx,
                };

                let users_to_send_to = self
                    .to_username
                    .as_ref()
                    .and_then(|username| self.username_to_user.get(username))
                    .map(|user| vec![user.clone()])
                    .unwrap_or_else(|| self.username_to_user.values().cloned().collect());

                self.link.send_message(Self::Message::Running {
                    users_to_send_to,
                    remaining: total_tx,
                });

                true
            }
            Self::Message::Running {
                users_to_send_to,
                mut remaining,
            } => {
                if let State::Started { total_tx, .. } = self.state {
                    let sender = self.props.user.0.clone();
                    let recipient = users_to_send_to.choose(&mut thread_rng()).unwrap(); // can't be empty
                    let base_sequence =
                        self.props.user.1 + (total_tx - remaining) as sieve::Sequence + 1;

                    let transfers_count = remaining.min(TRANSFER_PER_REFRESH);
                    for sequence in
                        (0..transfers_count).map(|i| base_sequence + i as sieve::Sequence)
                    {
                        self.send_asset_agent.send((
                            sender.clone(),
                            sequence,
                            recipient.clone(),
                            1,
                        ));
                    }
                    remaining -= transfers_count;

                    let callback = self.link.callback(|m| m);
                    if remaining > 0 {
                        // trigger refresh
                        Timeout::new(0, move || {
                            callback.emit(Self::Message::Running {
                                users_to_send_to,
                                remaining,
                            })
                        })
                        .forget();
                    }
                }

                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
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
                        value=self.amount.to_string()
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
                        { for self.sorted_usernames.iter().map(|username| html! {
                            <option>{ username.clone() }</option>
                        }) }
                    </select>
                 </label>

                <button
                    onclick=self.link.callback(|_| Self::Message::Start)
                    disabled=matches!(self.state, State::Started { .. })
                > { "Launch" } </button>
            </div>

            <hr />

            { if let State::Started { started_at, sent_tx, confirmed_tx, total_tx } = self.state {
                  Speedtest::view_speedtest(sent_tx, confirmed_tx, total_tx, Local::now() - started_at)
             } else if let State::Done { elapsed, total_tx } = self.state {
                  Speedtest::view_speedtest(total_tx, total_tx, total_tx, elapsed)
             } else {
                 html! {}
             } }


        </> }
    }
}

impl Speedtest {
    fn view_speedtest(
        sent_tx: usize,
        confirmed_tx: usize,
        total_tx: usize,
        elapsed: Duration,
    ) -> Html {
        let tps = (confirmed_tx as u64 * 1000)
            .checked_div(elapsed.num_milliseconds() as u64)
            .unwrap_or(0); // TODO show a "computing" text

        html! { <div style=concat!( "display: flex;", "flex-direction: column;")>
                { format!("Transactions sent: {}/{}", sent_tx, total_tx) }
                <br />
                { format!("Transactions confirmed_tx: {}/{}", confirmed_tx, total_tx) }

                <p> { format!("Running for {}s", elapsed.num_seconds()) } </p>

                { "AT2's TPS: " } { tps }
                <br />
                { "Bitcoin's TPS: 7" }
                <br />
                { "Ethereum's TPS: 25" }
            </div>
        }
    }
}
