use at2_ns::{proto::Account, Client};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use super::super::config::NAME_SERVICE_URI;

pub struct YourAccount {
    users: Vec<Account>,
}

pub enum Message {
    NewUsers(Vec<Account>),
}

impl YourAccount {
    fn render_user(user: &Account) -> Html {
        html! {
            <p> { &user.name } </p>
        }
    }
}

impl Component for YourAccount {
    type Properties = ();
    type Message = Message;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        spawn_local(async move {
            let mut client = Client::new(NAME_SERVICE_URI.parse().unwrap()); // TODO unwrap
            let users = client.get_all().await.unwrap(); // TODO unwrap

            link.callback(Self::Message::NewUsers).emit(users);
        });

        Self { users: Vec::new() }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Self::Message::NewUsers(users) => {
                self.users = users;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! { <div class=classes!("page")>
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

            { for self.users.iter().map(YourAccount::render_user) }

        </div> }
    }
}
