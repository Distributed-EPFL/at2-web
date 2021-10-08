mod new_account;
mod speedtest;
mod style;
mod summary;
mod welcome;
mod your_account;

use at2_ns::FullUser;
use drop::crypto::sign;
use new_account::NewAccount;
use speedtest::Speedtest;
pub use style::Style;
use summary::Summary;
use welcome::Welcome;
use yew::prelude::*;
use your_account::YourAccount;

pub enum Message {
    PreviousPage,
    NextPage,

    UserCreated(Box<FullUser>),
}

const PAGE_COUNT: usize = 5;

pub struct Pages {
    link: ComponentLink<Self>,
    index: usize,

    user: FullUser,
    user_created: bool,
}

impl Component for Pages {
    type Properties = ();
    type Message = Message;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            index: 0,

            user: FullUser::new("".to_owned(), sign::KeyPair::random()),
            user_created: false,
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
                self.user = *user;
                self.user_created = true;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! { <>
            { match self.index {
                0 => html! { <Welcome/> },
                1 => html! { <NewAccount
                    on_new_user=self.link.callback(Self::Message::UserCreated)
                    user=self.user.clone()
                    user_created=self.user_created
                /> },
                2 => html! { <YourAccount/> },
                3 => html! { <Speedtest/> },
                4 => html! { <Summary/> },
                _ => panic!("unreachable"),
            } }

            <div class=classes!("bottom")>
                <button
                    onclick=self.link.callback(|_| Self::Message::PreviousPage)
                    disabled=self.index == 0
                > { "Previous" } </button>
                <span>{ format!("{}/{}", self.index + 1, PAGE_COUNT) }</span>
                <button
                    onclick=self.link.callback(|_| Self::Message::NextPage)
                    disabled=
                        self.index+1 == PAGE_COUNT || (self.index == 1 && !self.user_created)
                > { "Next" } </button>
            </div>
        </> }
    }
}
