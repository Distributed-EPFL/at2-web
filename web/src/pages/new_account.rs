use at2_ns::{
    client::{self, Client},
    User,
};
use material_yew::{MatButton, MatTextField};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::config::Config;

#[derive(Properties, Clone, PartialEq)]
pub struct Properties {
    /// User to create
    pub user: User,
    /// Was the user already created
    pub user_created: bool,
    /// Where to send to created user with a potentially different name that the one given
    pub on_new_user: Callback<Box<User>>,
}

pub enum Message {
    SetUsername(String),
    CreateUser,
    UserPut(Result<Box<User>, client::Error>),
}

pub struct NewAccount {
    link: ComponentLink<Self>,
    properties: Properties,

    client: Client,

    create_user_error: Option<client::Error>,
}

impl Component for NewAccount {
    type Properties = Properties;
    type Message = Message;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        let conf = Config::parse();

        Self {
            link,
            properties,
            client: Client::new(conf.name_service()),
            create_user_error: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Self::Message::SetUsername(username) => {
                self.properties.user.name = username;
                true
            }
            Self::Message::CreateUser => {
                let (user, mut client) = (self.properties.user.clone(), self.client.clone());
                let callback = self.link.callback(Self::Message::UserPut);

                spawn_local(async move {
                    callback.emit(client.put(user.clone()).await.map(|_| Box::new(user)))
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
        let ret = properties != self.properties;

        self.properties = properties;

        ret
    }

    fn view(&self) -> Html {
        html! { <>
            <h1> { "New account" } </h1>

            <p> { "
                Here, you can create your own account.
                It will automatically be credited with a fixed amount of
                asset that you can pass around.
                This account is stored in your browser, so if you
                clear your sites' data, you won't be able to access it again
                (but you can recreate a new one).
            " } <br /> { "
                You can also see the various servers of the network below.
            " } </p>

            <hr />

            <div style=concat!(
                "display: flex;",
                "flex-direction: column;",
            )>
                { if !self.properties.user_created { html! { <p> {
                    "We generate a username for you, feel free to change it."
                } </p> } } else { html! {} } }

                <div style=concat!(
                    "display: flex;",
                    "justify-content: space-around;",
                    "align-items: center;",
                )>
                    <MatTextField
                        label="Enter your username"
                        oninput=self.link.callback(|event: InputData|
                            Self::Message::SetUsername(event.value))
                        value=self.properties.user.name.clone()
                    />

                    <span
                        onclick=self.link.callback(|_| Self::Message::CreateUser)
                    ><MatButton
                        label=if self.properties.user_created { "Update username" } else { "Create user" }
                        raised=true
                    /></span>
                </div>

                { self.create_user_error.as_ref().map(|err| html! {
                    <p style="color: red"> { format!("error while creating user: {}", err) } </p>
                }).unwrap_or_else(|| html! {}) }
            </div>

            <hr />

            <h2> { "Network" } </h2>

            <style> { "
                .boxes {
                    position: absolute;
                    width: 20em; height: 10em;
                    margin-left: 10em;
                    z-index: -1;

                    border: dotted var(--light-grey);
                }

                .boxes > * {
                    position: absolute;
                    margin: 0px;

                    background-color: white;
                    border: solid;
                    padding: 1em 2em;
                }
                @media (prefers-color-scheme: dark) {
                    .boxes > * {
                      background-color: var(--dark-grey);
                    }
                }

                .boxes > :nth-child(1) {
                    top: -2em; left: -4em;
                }
                .boxes > :nth-child(2) {
                    top: -2em; left: 20em;
                }
                .boxes > :nth-child(3) {
                    top: 10em; left: 8em;
                }
            " } </style>


            <div class=classes!("boxes")>
                <p> { "C4DT" } </p>
                <p> { "DCL" } </p>
                <p> { "ineiti" } </p>
            </div>
        </> }
    }
}
