use std::{net::SocketAddr, process};

use snafu::ResultExt;
use structopt::StructOpt;
use tonic::transport::Server;
use tracing::{subscriber, Level};
use tracing_fmt::FmtSubscriber;

mod accounts;
mod proto;
mod rpc;

#[derive(structopt::StructOpt)]
struct Arguments {
    address: SocketAddr,
}

#[derive(Debug, snafu::Snafu)]
enum Error {
    #[snafu(display("logging: {}", source))]
    Logging {
        source: tracing::dispatcher::SetGlobalDefaultError,
    },
    #[snafu(display("service: {}", source))]
    Service { source: tonic::transport::Error },
    #[snafu(display("rpc: {}", source))]
    Rpc { source: tonic::transport::Error },
}

async fn run(address: SocketAddr) -> Result<(), Error> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    subscriber::set_global_default(subscriber).context(Logging)?;

    let service = rpc::Service::new();

    Server::builder()
        .add_service(proto::DnsServer::new(service))
        .serve(address)
        .await
        .context(Rpc)?;

    Ok(())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let arguments = Arguments::from_args();

    run(arguments.address).await.unwrap_or_else(|err| {
        eprintln!("error running cmd: {}", err);
        process::exit(1);
    });
}
