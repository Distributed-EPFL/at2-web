mod new_account;
mod speedtest;
mod style;
mod summary;
mod welcome;
mod your_account;

use at2_ns::User;
use drop::crypto::sign;
use material_yew::MatButton;
use new_account::NewAccount;
use speedtest::Speedtest;
pub use style::Style;
use summary::Summary;
use welcome::Welcome;
use yew::{
    format::Json,
    prelude::*,
    services::storage::{Area, StorageService},
};
use your_account::YourAccount;

pub enum Message {
    PreviousPage,
    NextPage,

    UserCreated(Box<User>),
    SequenceBumped(sieve::Sequence),
}

const PAGE_COUNT: usize = 5;
const STORAGE_KEY: &str = "at2-user";

/// Component showing the pages
pub struct Pages {
    link: ComponentLink<Self>,
    index: usize,

    user: (User, sieve::Sequence),
    user_created: bool,
}

impl Component for Pages {
    type Properties = ();
    type Message = Message;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let (user, user_created) = if let Ok(Json(Ok(stored))) =
            StorageService::new(Area::Local).map(|storage| storage.restore::<Json<_>>(STORAGE_KEY))
        {
            (stored, true)
        } else {
            (
                (User::new("".to_owned(), sign::KeyPair::random()), 0),
                false,
            )
        };

        Self {
            link,
            index: 0,

            user,
            user_created,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::PreviousPage => {
                self.index -= 1;
                true
            }
            Self::Message::NextPage => {
                self.index += 1;
                true
            }
            Self::Message::UserCreated(user) => {
                self.user.0 = *user;
                self.user_created = true;

                if let Ok(mut storage) = StorageService::new(Area::Local) {
                    storage.store(STORAGE_KEY, Json(&self.user));
                };

                true
            }
            Self::Message::SequenceBumped(seq) => {
                self.user.1 = seq;

                if let Ok(mut storage) = StorageService::new(Area::Local) {
                    storage.store(STORAGE_KEY, Json(&self.user));
                };

                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
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

            <div class=classes!("page")>
                { match self.index {
                    0 => html! { <Welcome/> },
                    1 => html! { <NewAccount
                        on_new_user=self.link.callback(Self::Message::UserCreated)
                        user=self.user.0.clone()
                        user_created=self.user_created
                    /> },
                    2 => html! { <YourAccount
                        user=self.user.clone()
                        bump_sequence=self.link.callback(Self::Message::SequenceBumped)
                    /> },
                    3 => html! { <Speedtest
                        user=self.user.clone()
                        bump_sequence=self.link.callback(Self::Message::SequenceBumped)
                    /> },
                    4 => html! { <Summary/> },
                    _ => panic!("unreachable"),
                } }
            </div>

            <div class=classes!("bottom")>
                <div>
                    <span onclick=self.link.callback(|_| Self::Message::PreviousPage)>
                        <MatButton
                            label="Previous"
                            raised=true
                            disabled=self.index == 0
                        />
                    </span>
                    <span>{ format!("{}/{}", self.index + 1, PAGE_COUNT) }</span>
                    <span onclick=self.link.callback(|_| Self::Message::NextPage)>
                        <MatButton
                            label="Next"
                            raised=true
                            disabled=
                                self.index+1 == PAGE_COUNT || (self.index == 1 && !self.user_created)
                        />
                    </span>
                </div>
            </div>
        </> }
    }
}
