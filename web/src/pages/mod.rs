mod new_account;
mod speedtest;
mod style;
mod summary;
mod welcome;
mod your_account;

use std::cmp::min;

use at2_node::client;
use at2_ns::User;
use drop::crypto::sign;
use gloo_storage::LocalStorage;
use material_yew::MatButton;
use new_account::NewAccount;
use speedtest::Speedtest;
pub use style::Style;
use summary::Summary;
use welcome::Welcome;
use yew::prelude::*;
use yew_agent::Bridge;
use your_account::YourAccount;

use crate::agents;

pub enum Message {
    PreviousPage,
    NextPage,

    UserCreated(Box<User>),
    SequenceMightBump(Result<sieve::Sequence, client::Error>),
    SequenceBumped(sieve::Sequence),
}

const PAGE_COUNT: usize = 5;
const STORAGE_KEY: &str = "at2-user";

/// Component showing the pages
pub struct Pages {
    index: usize,

    user: (User, sieve::Sequence),
    user_created: bool,

    get_last_sequence_agent: Box<dyn Bridge<agents::GetLastSequence>>,
}

impl Component for Pages {
    type Properties = ();
    type Message = Message;

    fn create(ctx: Context<Self>) -> Self {
        let mut get_last_sequence_agent =
            agents::GetLastSequence::bridge(ctx.link().callback(Self::Message::SequenceMightBump));

        let (user, user_created) = if let Ok(stored) = LocalStorage::get(STORAGE_KEY) {
            let (user, seq): (User, sieve::Sequence) = stored;
            get_last_sequence_agent.send(user.clone().to_thin());

            ((user, seq), true)
        } else {
            (
                (
                    User::new(
                        names::Generator::default().next().unwrap(), // can't fail
                        sign::KeyPair::random(),
                    ),
                    0,
                ),
                false,
            )
        };

        Self {
            index: 2,

            user,
            user_created,

            get_last_sequence_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::PreviousPage => {
                self.index = self.index.saturating_sub(1);
                true
            }
            Self::Message::NextPage => {
                self.index = min(self.index + 1, PAGE_COUNT - 1);
                true
            }
            Self::Message::UserCreated(user) => {
                self.user.0 = *user;
                self.user_created = true;

                LocalStorage.set(STORAGE_KEY, &self.user);

                true
            }
            Self::Message::SequenceMightBump(ret) => {
                if let Ok(seq) = ret {
                    self.link.send_message(Self::Message::SequenceBumped(seq));
                } else {
                    self.get_last_sequence_agent
                        .send(self.user.0.clone().to_thin());
                }

                false
            }
            Self::Message::SequenceBumped(seq) => {
                self.user.1 = seq;

                LocalStorage.set(STORAGE_KEY, &self.user);

                true
            }
        }
    }

    fn view(&self) -> Html {
        html! { <>
            <style> { "
            .bottom {
              position: fixed;
              bottom: 0;
              width: 100%;

              display: flex;
              justify-content: space-around;

              border-top: solid lightgrey;

              background-color: white;

              /* override base CSS */
              margin: 0;
            }
            @media (prefers-color-scheme: dark) {
                .bottom {
                  background-color: inherit;
                }
            }
            .bottom > div {
              align-items: center;
              display: flex;
              justify-content: space-around;
            }
            .bottom > div > * {
              margin: 1em;
            }

            .page {
                text-align: center;

                width: 40em;
                margin: 10em auto;
            }
            .page > * {
                margin: 2em auto;
            }
            .page > p {
                text-align: justify;
            }
            " } </style>

            <div class="page" hidden={ self.index != 0 }>
                <Welcome/>
            </div>
            <div class="page" hidden={ self.index != 1 }>
                <NewAccount
                    on_new_user={ self.link.callback(Self::Message::UserCreated) }
                    user={ self.user.0.clone() }
                    user_created={ self.user_created }
                />
            </div>
            <div class="page" hidden={ self.index != 2 }>
                 <YourAccount
                    user={ self.user.clone() }
                    bump_sequence={ self.link.callback(Self::Message::SequenceBumped) }
                />
            </div>
            <div class="page" hidden={ self.index != 3 }>
                <Speedtest
                    user={ self.user.clone() }
                    bump_sequence={ self.link.callback(Self::Message::SequenceBumped) }
                />
            </div>
            <div class="page" hidden={ self.index != 4 }>
                <Summary/>
            </div>

            <div class="bottom">
                <div>
                    <span onclick={ self.link.callback(|_| Self::Message::PreviousPage) }>
                        <MatButton
                            label="Previous"
                            raised=true
                            disabled={ self.index == 0 }
                        />
                    </span>
                    <span>{ format!("{}/{}", self.index + 1, PAGE_COUNT) }</span>
                    <span onclick={ self.link.callback(|_| Self::Message::NextPage) }>
                        <MatButton
                            label="Next"
                            raised=true
                            disabled={ self.index+1 == PAGE_COUNT || (self.index == 1 && !self.user_created) }
                        />
                    </span>
                </div>
            </div>
        </> }
    }
}
