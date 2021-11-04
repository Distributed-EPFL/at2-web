use at2_node::client::{self, Client};
use at2_ns::ThinUser;
use wasm_bindgen_futures::spawn_local;
use yew::worker::*;

use crate::config::Config;

/// Get the balance of a user
pub struct GetBalance {
    link: AgentLink<Self>,
    client: Client,
}

impl Agent for GetBalance {
    type Reach = Context<Self>;
    type Message = (HandlerId, Result<u64, client::Error>);
    type Input = ThinUser;
    type Output = Result<u64, client::Error>;

    fn create(link: AgentLink<Self>) -> Self {
        let conf = Config::parse().unwrap(); // TODO unwrap

        Self {
            link,
            client: Client::new(conf.network().to_owned()).unwrap(), // TODO unwrap
        }
    }

    fn update(&mut self, (id, ret): Self::Message) {
        self.link.respond(id, ret);
    }

    fn handle_input(&mut self, user: Self::Input, id: HandlerId) {
        let mut client = self.client.clone();
        let callback = self.link.callback(|ret| ret);

        spawn_local(async move {
            callback.emit((id, client.get_balance(user.public_key()).await));
        });
    }
}
