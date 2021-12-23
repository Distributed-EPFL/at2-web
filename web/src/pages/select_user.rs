use std::collections::HashMap;

use at2_ns::Contact;
use yew::{prelude::*, worker::Agent};

use crate::agents;

#[derive(Properties, Clone)]
pub struct Properties {
    /// Where to send the new sequence when the current one is used
    pub user_selected: Callback<Contact>,
}

pub struct SelectUser {
    link: ComponentLink<Self>,

    props: Properties,

    #[allow(dead_code)] // never dropped
    get_users_agent: Box<dyn Bridge<agents::GetUsers>>,
    sorted_usernames: Vec<String>,
    username_to_user: HashMap<String, Contact>,
}

pub enum Message {
    GotUsers(<agents::GetUsers as Agent>::Output),

    SelectUsername(usize),
}

impl Component for SelectUser {
    type Properties = Properties;
    type Message = Message;

    fn create(props: Properties, link: ComponentLink<Self>) -> Self {
        let get_users_agent = agents::GetUsers::bridge(link.callback(Message::GotUsers));

        Self {
            link,
            props,

            get_users_agent,
            sorted_usernames: Vec::new(),
            username_to_user: HashMap::new(),
        }
    }

    fn update(&mut self, message: Message) -> ShouldRender {
        match message {
            Message::GotUsers(users) => {
                let was_empty = self.sorted_usernames.is_empty();

                let mut sorted_usernames = users
                    .iter()
                    .map(|user| user.name.clone())
                    .collect::<Vec<_>>();
                sorted_usernames.sort_unstable();
                self.sorted_usernames = sorted_usernames;

                self.username_to_user = users
                    .iter()
                    .cloned()
                    .map(|user| (user.name.clone(), user))
                    .collect();

                if was_empty {
                    self.link.send_message(Message::SelectUsername(0));
                }

                true
            }
            Message::SelectUsername(index) => {
                if let Some(user) = self
                    .sorted_usernames
                    .get(index)
                    .and_then(|username| self.username_to_user.get(username))
                {
                    self.props.user_selected.emit(user.clone())
                }

                false
            }
        }
    }

    fn change(&mut self, _: Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <select
                onchange=self.link.callback(|event: ChangeData| match event {
                    ChangeData::Select(elem) => Message::SelectUsername(elem.selected_index() as usize),
                    _ => unreachable!(),
                })
            >
                { self.sorted_usernames.iter().map(|username| html! {
                    <option>{ username.clone() }</option>
                }).collect::<Html>() }
            </select>
        }
    }
}
