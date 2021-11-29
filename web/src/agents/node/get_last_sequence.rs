use at2_node::client::{self, Client};
use at2_ns::Contact;
use wasm_bindgen_futures::spawn_local;
use yew::worker::*;

use crate::config::Config;

/// Get the last sequence used by a user.
///
/// Every transactions is sent alongside the public key of user sending it and
/// a sequence number (a sender-incremented counter). This agent retrieves the latest processed
/// sequence for a given user.
pub struct GetLastSequence {
    link: AgentLink<Self>,
    client: Client,
}

impl Agent for GetLastSequence {
    type Reach = Context<Self>;
    type Message = (HandlerId, Result<sieve::Sequence, client::Error>);
    type Input = Contact;
    type Output = Result<sieve::Sequence, client::Error>;

    fn create(link: AgentLink<Self>) -> Self {
        let conf = Config::parse();

        Self {
            link,
            client: Client::new(conf.network().to_owned()),
        }
    }

    fn update(&mut self, (id, ret): Self::Message) {
        self.link.respond(id, ret);
    }

    fn handle_input(&mut self, user: Self::Input, id: HandlerId) {
        let mut client = self.client.clone();
        let callback = self.link.callback(|ret| ret);

        spawn_local(async move {
            callback.emit((id, client.get_last_sequence(user.public_key()).await));
        });
    }
}
