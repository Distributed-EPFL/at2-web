use at2_ns::{Client, User};
use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, services::ConsoleService};

#[derive(Properties, Clone, PartialEq)]
pub struct Properties {
    pub on_new_user: Callback<Box<User>>,
}

pub enum Message {
    SetUsername(String),
    CreateUser,
    UserCreated(Box<User>),
}

enum UserState {
    Username(String),
    Created,
}

pub struct NewAccount {
    link: ComponentLink<Self>,
    properties: Properties,

    user_state: UserState,
}

const NAME_SERVICE_URI: &str = "http://localhost:6263";

impl Component for NewAccount {
    type Properties = Properties;
    type Message = Message;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            properties,
            user_state: UserState::Username("".to_owned()),
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Self::Message::SetUsername(username) => {
                match self.user_state {
                    UserState::Username(_) => self.user_state = UserState::Username(username),
                    UserState::Created => panic!("user already created"),
                }
                false
            }
            Self::Message::CreateUser => {
                match self.user_state {
                    UserState::Username(ref username) => {
                        let username = username.to_owned();
                        let callback = self.link.callback(Self::Message::UserCreated);

                        spawn_local(async move {
                            let user = User::new(username.clone());

                            let mut client = Client::new(NAME_SERVICE_URI.parse().unwrap());
                            client.put(&user).await.unwrap(); // TODO unwrap

                            callback.emit(Box::new(user));
                        });

                        self.user_state = UserState::Created;
                    }
                    UserState::Created => panic!("user already created"),
                }
                true
            }
            Self::Message::UserCreated(user) => {
                ConsoleService::info(&format!("new user: {:?}", user));
                false
            }
        }
    }

    fn change(&mut self, properties: Self::Properties) -> ShouldRender {
        properties != self.properties
    }

    fn view(&self) -> Html {
        html! { <div class=classes!("page")>
            <h1> { "New account" } </h1>

            <p>
                { "Here, you can create your own account." }
                <br />
                { "You can also see the status of the network below." }
            </p>

            <hr />

            <label>
                { "Enter your username" }
                <input
                    oninput=self.link.callback(|event: InputData|
                        Self::Message::SetUsername(event.value))
                    disabled=matches!(self.user_state, UserState::Created)
                    type={ "text" }
                />
            </label>

            <button
                onclick=self.link.callback(|_| Self::Message::CreateUser)
                disabled=matches!(self.user_state, UserState::Created)
            > { "Create user" } </button>

            <hr />

            <h2> { "Network" } </h2>
        </div> }
    }
}
