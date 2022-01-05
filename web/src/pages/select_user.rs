use std::{collections::HashMap, iter};

use at2_ns::Contact;
use material_yew::{
    select::{ListIndex, SelectedDetail},
    MatListItem, MatSelect,
};
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
            <MatSelect
                onselected=self.link.callback(|detail: SelectedDetail| match detail.index {
                    ListIndex::Single(Some(elem)) => Message::SelectUsername(elem),
                    _ => unreachable!(),
                })
            >
                { self.sorted_usernames.iter()
                    .zip(iter::once(true).chain(iter::repeat(false)))
                    .map(|(username, selected)| html! {
                    <MatListItem selected=selected>{ username.clone() }</MatListItem>
                }).collect::<Html>() }
            </MatSelect>
        }
    }
}
