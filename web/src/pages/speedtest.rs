use at2_ns::{Contact, User};
use chrono::{offset::Local, DateTime, Duration};
use gloo_timers::callback::{Interval, Timeout};
use material_yew::{MatButton, MatLinearProgress};
use yew::{prelude::*, worker::Agent};

use super::select_user::SelectUser;
use crate::agents;

const TRANSFER_PER_REFRESH: usize = 50;

#[derive(Properties, Clone)]
pub struct Properties {
    /// User's account
    pub user: (User, sieve::Sequence),
    /// Where to send the new sequence when the current one is used
    pub bump_sequence: Callback<sieve::Sequence>,
}

fn validate_amount(amount: &str) -> Option<usize> {
    amount.parse::<usize>().ok()
}

pub struct Speedtest {
    link: ComponentLink<Self>,
    props: Properties,

    send_asset_agent: Box<dyn Bridge<agents::SendAsset>>,

    amount: String,
    to_user: Option<Contact>,

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
    GotLastSequence(<agents::GetLastSequence as Agent>::Output),

    UpdateTransactionAmount(String),
    SelectUser(Contact),

    Start,
    Running {
        user_to_send_to: Contact,
        remaining: usize,
    },
}

impl Component for Speedtest {
    type Properties = Properties;
    type Message = Message;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let send_asset_agent =
            agents::SendAsset::bridge(link.callback(Self::Message::TransactionSent));

        let user = props.user.0.clone().to_thin();
        let mut get_last_sequence_agent =
            agents::GetLastSequence::bridge(link.callback(Self::Message::GotLastSequence));

        Self {
            link,
            props,

            send_asset_agent,

            amount: "1000".to_owned(),
            to_user: None,

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

            Self::Message::GotLastSequence(ret) => {
                if let State::Started {
                    started_at,
                    confirmed_tx,
                    total_tx,
                    ..
                } = &mut self.state
                {
                    if let Ok(seq) = ret {
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
                self.amount = amount;
                false
            }
            Self::Message::SelectUser(username) => {
                self.to_user = Some(username);
                false
            }

            Self::Message::Start => {
                if let Some(total_tx) = validate_amount(&self.amount) {
                    if let Some(user_to_send_to) = self.to_user.clone() {
                        self.state = State::Started {
                            started_at: Local::now(),
                            sent_tx: 0,
                            confirmed_tx: 0,
                            total_tx,
                        };

                        self.link.send_message(Self::Message::Running {
                            user_to_send_to,
                            remaining: total_tx,
                        });
                    }
                }

                true
            }
            Self::Message::Running {
                user_to_send_to,
                mut remaining,
            } => {
                if let State::Started { total_tx, .. } = self.state {
                    let sender = self.props.user.0.clone();
                    let base_sequence =
                        self.props.user.1 + (total_tx - remaining) as sieve::Sequence + 1;

                    let transfers_count = remaining.min(TRANSFER_PER_REFRESH);
                    for sequence in
                        (0..transfers_count).map(|i| base_sequence + i as sieve::Sequence)
                    {
                        self.send_asset_agent.send((
                            sender.clone(),
                            sequence,
                            user_to_send_to.clone(), // can't be empty
                            1,
                        ));
                    }
                    remaining -= transfers_count;

                    let callback = self.link.callback(|m| m);
                    if remaining > 0 {
                        // trigger refresh
                        Timeout::new(0, move || {
                            callback.emit(Self::Message::Running {
                                user_to_send_to,
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

            <p> { "
                One of the most interesting features of AT2 is that it allows
                for many " }
                <b>{ "more transactions per second" }</b>
                { " (TPS) than the two most
                popular blockchains, such as Bitcoin or Ethereum.
                Indeed, its TPS scales in " }
                <code class="math">{ "1/log(#nodes)" }</code>
                { " meaning that the TPS goes down as more nodes join the
                network but the change in speed gets less and less
                important.
                The two others chains actually have a fixed max TPS, so it
                doesn't scale at all.
            " } <br />
                { "Here you can actually " }
                <b>{ "test it" }</b>
                { ", flooding the network with
                transactions, and see how well it compares to the two.
            " } <br /> { "
                Bear in mind that the transactions are sent by your browser to
                a node, which then forwards them to the actual network.
                This greatly reduces the computed TPS as opposed to sending
                your transactions on the network directly as you would do in
                a more realistic use case. It means that the TPS measured here
                is below the speed reported in the " }
                <a href="https://arxiv.org/abs/1812.10844" target="_blank"> { "research paper" } </a>
                { "." }
             </p>

            <hr />

            <span style=concat!(
                "display: flex;",
                "align-items: center;",
            )>

                <label>
                    { "How many transactions to send " }
                    <input
                        oninput=self.link.callback(|event: InputData|
                            Self::Message::UpdateTransactionAmount(event.value))
                        value=self.amount.clone()
                        min=1
                        type={ "number" }
                    />
                </label>

                <label>
                    { "To whom to send to " }
                    <SelectUser
                        user_selected=self.link.callback(Message::SelectUser)
                    />
                </label>

                <span
                    onclick=self.link.callback(|_| Self::Message::Start)
                ><MatButton
                    label="Launch"
                    raised=true
                    disabled=matches!(self.state, State::Started { .. }) || validate_amount(&self.amount).is_none()
                /></span>
            </span>

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
    fn progress_bar(progress: f32) -> Html {
        assert!(
            (0.0..=1.0).contains(&progress),
            "progress out of range: {:?}",
            progress
        );

        // MatLinearProgress is broken, this workaround looks fine

        html! {
            <span style="display: block; width: 20em;">
                <MatLinearProgress
                    buffer=1.0
                    reverse=true
                    progress=2.0 - 2.0*progress
                />
            </span>
        }
    }

    fn view_speedtest(
        sent_tx: usize,
        confirmed_tx: usize,
        total_tx: usize,
        elapsed: Duration,
    ) -> Html {
        let tps = (confirmed_tx as u64 * 1000).checked_div(elapsed.num_milliseconds() as u64);

        const FIRST_COL: &str = "text-align: end; padding: 0 1em";

        html! { <table>
            <tr>
                <td style=FIRST_COL> { "Transactions sent" } </td>
                <td> { Speedtest::progress_bar(sent_tx as f32/total_tx as f32) } </td>
            </tr>
            <tr>
                <td style=FIRST_COL> { "Transactions confirmed" } </td>
                <td> { Speedtest::progress_bar(confirmed_tx as f32/total_tx as f32) } </td>
            </tr>

            <tr>
                <td style=FIRST_COL> { "Running for" } </td>
                <td> { format!("{:.1}s", elapsed.num_milliseconds() as f64 / 1000.0) } </td>
            </tr>

            <tr>
                <td style=FIRST_COL> { "AT2's computed TPS" } </td>
                <td> { tps.unwrap_or(0) } </td>
            </tr>
            <tr>
                <td style=FIRST_COL> { "Bitcoin's TPS" } </td>
                <td> { 7 } </td>
            </tr>
            <tr>
                <td style=FIRST_COL> { "Ethereum's TPS" } </td>
                <td> { 25 } </td>
            </tr>
        </table> }
    }
}
