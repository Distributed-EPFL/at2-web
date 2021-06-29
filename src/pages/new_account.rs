use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct Properties {
    pub validate: Callback<bool>,
}

pub enum Message {
    ValidateUsername(String),
}

pub struct NewAccount {
    link: ComponentLink<Self>,
    validate: Callback<bool>,
}

impl Component for NewAccount {
    type Properties = Properties;
    type Message = Message;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            validate: properties.validate,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Self::Message::ValidateUsername(username) => {
                self.validate.emit(!username.is_empty());
                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
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
                        Self::Message::ValidateUsername(event.value))
                    type={ "text" }
                />
            </label>

            <hr />

            <h2> { "Network" } </h2>
        </div> }
    }
}
