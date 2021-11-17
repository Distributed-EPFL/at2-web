use std::collections::HashSet;

use at2_node::{client::Client, FullTransaction};
use gloo_timers::callback::Interval;
use wasm_bindgen_futures::spawn_local;
use yew::{services::ConsoleService, worker::*};

use crate::config::Config;

/// Get the latest processed transactions
pub struct GetLatestTransactions {
    link: AgentLink<Self>,

    #[allow(dead_code)] // drop when agent is destroyed
    refresher: Interval,
    last_send: Vec<FullTransaction>,

    subscribers: HashSet<HandlerId>,
    subscribers_changed_since_last_sent: bool,
}

impl Agent for GetLatestTransactions {
    type Reach = Context<Self>;
    type Message = Vec<FullTransaction>;
    type Input = ();
    type Output = Vec<FullTransaction>;

    fn create(link: AgentLink<Self>) -> Self {
        let conf = Config::parse();
        let client = Client::new(conf.network().to_owned());
        let update_txs = link.callback(|users| users);

        Self {
            link,
            refresher: Interval::new(1_000, move || {
                let (mut client, update_txs) = (client.clone(), update_txs.clone());

                spawn_local(async move {
                    match client.get_latest_transactions().await {
                        Ok(users) => update_txs.emit(users),
                        Err(err) => {
                            ConsoleService::error(&format!("unable to refresh users: {}", err))
                        }
                    }
                });
            }),
            last_send: Vec::new(),
            subscribers: HashSet::new(),
            subscribers_changed_since_last_sent: false,
        }
    }

    fn update(&mut self, txs: Self::Message) {
        if txs.ne(&self.last_send) {
            self.subscribers
                .iter()
                .for_each(|id| self.link.respond(*id, txs.clone()));

            self.last_send = txs;
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
