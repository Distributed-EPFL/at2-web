use std::collections::HashSet;

use at2_ns::{client::Client, ThinUser};
use gloo_timers::callback::Interval;
use wasm_bindgen_futures::spawn_local;
use yew::{services::ConsoleService, worker::*};

use crate::config::Config;

pub struct UsersAgent {
    link: AgentLink<Self>,

    last_send: HashSet<ThinUser>,

    subscribers: HashSet<HandlerId>,
}

impl Agent for UsersAgent {
    type Reach = Context<Self>;
    type Message = HashSet<ThinUser>;
    type Input = ();
    type Output = HashSet<ThinUser>;

    fn create(link: AgentLink<Self>) -> Self {
        let conf = Config::parse().unwrap(); // TODO unwrap
        let client = Client::new(conf.name_service());
        let update_users = link.callback(|users| users);

        let ret = Self {
            link,
            last_send: HashSet::new(),
            subscribers: HashSet::new(),
        };

        Interval::new(1_000, move || {
            let (mut client, update_users) = (client.clone(), update_users.clone());

            spawn_local(async move {
                match client.get_all().await {
                    Ok(users) => update_users.emit(users),
                    Err(err) => ConsoleService::error(&format!("unable to refresh users: {}", err)),
                }
            });
        })
        .forget();

        ret
    }

    fn update(&mut self, users: Self::Message) {
        if users.symmetric_difference(&self.last_send).next() != None {
            self.subscribers
                .iter()
                .for_each(|id| self.link.respond(*id, users.clone()))
        }
    }

    fn handle_input(&mut self, _: Self::Input, _: HandlerId) {
        panic!("do not support input, only subscribing");
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
