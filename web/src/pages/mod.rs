mod new_account;
mod speedtest;
mod style;
mod welcome;
mod your_account;

use at2_ns::FullUser;
use new_account::NewAccount;
use speedtest::Speedtest;
pub use style::Style;
use welcome::Welcome;
use yew::prelude::*;
use your_account::YourAccount;

pub enum Message {
    PreviousPage,
    NextPage,

    UserCreated(Box<FullUser>),
}

const PAGE_COUNT: usize = 4;

pub struct Pages {
    link: ComponentLink<Self>,
    index: usize,

    user: Option<FullUser>,
}

impl Component for Pages {
    type Properties = ();
    type Message = Message;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            index: 0,

            user: None,
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
                debug_assert!(matches!(self.user, None));
                self.user = Some(*user);
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! { <>
            <div hidden=self.index != 0> <Welcome/> </div>
            <div hidden=self.index != 1> <NewAccount
                on_new_user=self.link.callback(Self::Message::UserCreated)
            /> </div>
            <div hidden=self.index != 2> <YourAccount/> </div>
            <div hidden=self.index != 3> <Speedtest/> </div>

            <div class=classes!("bottom")>
                <button
                    onclick=self.link.callback(|_| Self::Message::PreviousPage)
                    disabled=self.index == 0
                > { "Previous" } </button>
                <span>{ format!("{}/{}", self.index + 1, PAGE_COUNT) }</span>
                <button
                    onclick=self.link.callback(|_| Self::Message::NextPage)
                    disabled=
                        self.index+1 == PAGE_COUNT || (self.index == 1 && self.user.is_none())
                > { "Next" } </button>
            </div>
        </> }
    }
}
