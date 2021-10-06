use at2_ns::{
    client::{self, Client},
    FullUser,
};
use drop::crypto::sign;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::config::Config;

#[derive(Properties, Clone, PartialEq)]
pub struct Properties {
    pub on_new_user: Callback<Box<FullUser>>,
}

pub enum Message {
    SetUsername(String),
    CreateUser,
    UserPut(Result<Box<FullUser>, client::Error>),
}

pub struct NewAccount {
    link: ComponentLink<Self>,
    properties: Properties,

    client: Client,

    username: String,
    keypair: sign::KeyPair,
    create_user_error: Option<client::Error>,
}

impl Component for NewAccount {
    type Properties = Properties;
    type Message = Message;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        let conf = Config::parse().unwrap(); // TODO unwrap

        Self {
            link,
            properties,
            client: Client::new(conf.name_service()),
            username: "".to_owned(),
            keypair: sign::KeyPair::random(),
            create_user_error: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Self::Message::SetUsername(username) => {
                self.username = username;
                true
            }
            Self::Message::CreateUser => {
                let user = FullUser::new(self.username.clone(), self.keypair.clone());

                let mut client = self.client.clone();
                let callback = self.link.callback(Self::Message::UserPut);

                spawn_local(async move {
                    callback.emit(client.put(&user).await.map(|_| Box::new(user)))
                });

                false
            }
            Self::Message::UserPut(res) => {
                match res {
                    Ok(user) => {
                        self.properties.on_new_user.emit(user);
                        self.create_user_error = None;
                    }
                    Err(err) => {
                        self.create_user_error = Some(err);
                    }
                }

                true
            }
        }
    }

    fn change(&mut self, properties: Self::Properties) -> ShouldRender {
        properties != self.properties
    }

    fn view(&self) -> Html {
        // TODO fetch network
        let network = ["C4DT", "DCL", "ineiti"];

        html! { <div class=classes!("page")>
            <h1> { "New account" } </h1>

            <p>
                { "Here, you can create your own account." }
                <br />
                { "You can also see the status of the network below." }
            </p>

            <hr />

            <div style=concat!(
                "display: flex;",
                "flex-direction: column;",
                "align-items: center;",
            )>
                <div style="display: flex; padding: 1em 0em">
                    <label> { "Enter your username" } </label>
                    <input
                        oninput=self.link.callback(|event: InputData|
                            Self::Message::SetUsername(event.value))
                        type={ "text" }
                    />
                </div>

                <button
                    onclick=self.link.callback(|_| Self::Message::CreateUser)
                > { "Create user" } </button>

                { self.create_user_error.as_ref().map(|err| html! {
                    <p> { format!("error while creating user: {}", err) } </p>
                }).unwrap_or_else(|| html! {}) }
            </div>

            <hr />

            <h2> { "Network" } </h2>
            <span class=classes!("boxes") >
                { for network.iter().map(|node| html! {
                    <p> { node } </p>
                }) }
            </span>
        </div> }
    }
}
