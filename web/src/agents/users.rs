use std::collections::HashSet;

use at2_ns::{client::Client, ThinUser};
use gloo_timers::callback::Interval;
use wasm_bindgen_futures::spawn_local;
use yew::{services::ConsoleService, worker::*};

use crate::config::Config;

/// Get the users registered on the name service
pub struct GetUsers {
    link: AgentLink<Self>,

    #[allow(dead_code)] // drop when agent is destroyed
    refresher: Interval,
    last_send: HashSet<ThinUser>,

    subscribers: HashSet<HandlerId>,
    subscribers_changed_since_last_sent: bool,
}

impl Agent for GetUsers {
    type Reach = Context<Self>;
    type Message = HashSet<ThinUser>;
    type Input = ();
    type Output = HashSet<ThinUser>;

    fn create(link: AgentLink<Self>) -> Self {
        let conf = Config::parse();
        let client = Client::new(conf.name_service());
        let update_users = link.callback(|users| users);

        Self {
            link,
            refresher: Interval::new(1_000, move || {
                let (mut client, update_users) = (client.clone(), update_users.clone());

                spawn_local(async move {
                    match client.get_all().await {
                        Ok(users) => update_users.emit(users),
                        Err(err) => {
                            ConsoleService::error(&format!("unable to refresh users: {}", err))
                        }
                    }
                });
            }),
            last_send: HashSet::new(),
            subscribers: HashSet::new(),
            subscribers_changed_since_last_sent: false,
        }
    }

    fn update(&mut self, users: Self::Message) {
        if users.symmetric_difference(&self.last_send).next() != None
            || self.subscribers_changed_since_last_sent
        {
            self.subscribers
                .iter()
                .for_each(|id| self.link.respond(*id, users.clone()));

            self.last_send = users;
            self.subscribers_changed_since_last_sent = false;
        }
    }

    fn handle_input(&mut self, _: Self::Input, _: HandlerId) {
        panic!("do not support input, only subscribing");
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
        self.subscribers_changed_since_last_sent = true;
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
        self.subscribers_changed_since_last_sent = true;
    }
}
