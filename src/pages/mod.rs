mod style;

pub use style::Style;

mod new_account;
mod welcome;
mod your_account;

use new_account::NewAccount;
use welcome::Welcome;
use your_account::YourAccount;

use yew::prelude::*;

pub enum Message {
    PreviousPage,
    NextPage,

    ValidateNewAccount(bool),
}

pub struct Pages {
    link: ComponentLink<Self>,
    index: usize,

    new_account_is_valid: bool,
}

impl Component for Pages {
    type Properties = ();
    type Message = Message;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            index: 0,
            new_account_is_valid: false,
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
            Self::Message::ValidateNewAccount(is_valid) => {
                self.new_account_is_valid = is_valid;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        const PAGE_COUNT: usize = 3;

        html! { <>
            <div hidden=self.index != 0> <Welcome/> </div>
            <div hidden=self.index != 1> <NewAccount
                validate=self.link.callback(Self::Message::ValidateNewAccount)
            /> </div>
            <div hidden=self.index != 2> <YourAccount/> </div>

            <div class=classes!("bottom")>
                <button
                    onclick=self.link.callback(|_| Self::Message::PreviousPage)
                    disabled=self.index == 0
                > { "Previous" } </button>
                <span>{ format!("{}/{}", self.index + 1, PAGE_COUNT) }</span>
                <button
                    onclick=self.link.callback(|_| Self::Message::NextPage)
                    disabled=
                        self.index+1 == PAGE_COUNT ||
                        (self.index == 1 && !self.new_account_is_valid)
                > { "Next" } </button>
            </div>
        </> }
    }
}
