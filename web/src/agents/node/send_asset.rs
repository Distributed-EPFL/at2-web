use at2_node::client::{self, Client};
use at2_ns::{FullUser, ThinUser};
use wasm_bindgen_futures::spawn_local;
use yew::worker::*;

use crate::config::Config;

/// Send an asset on the network
pub struct SendAsset {
    link: AgentLink<Self>,
    client: Client,
}

impl Agent for SendAsset {
    type Reach = Context<Self>;
    type Message = (HandlerId, Result<(), client::Error>);
    type Input = (FullUser, sieve::Sequence, ThinUser, u64);
    type Output = Result<(), client::Error>;

    fn create(link: AgentLink<Self>) -> Self {
        let conf = Config::parse();

        Self {
            link,
            client: Client::new(conf.network().to_owned()).unwrap(), // TODO unwrap
        }
    }

    fn update(&mut self, (id, ret): Self::Message) {
        self.link.respond(id, ret);
    }

    fn handle_input(&mut self, (user, sequence, recipient, amount): Self::Input, id: HandlerId) {
        let mut client = self.client.clone();
        let callback = self.link.callback(|ret| ret);

        spawn_local(async move {
            callback.emit((
                id,
                client
                    .send_asset(
                        user.keypair(),
                        sequence,
                        recipient.public_key().to_owned(),
                        amount,
                    )
                    .await,
            ));
        });
    }
}
