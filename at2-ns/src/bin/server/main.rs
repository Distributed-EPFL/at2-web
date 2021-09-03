use std::{net::SocketAddr, process};

use at2_ns::proto::name_service_server::NameServiceServer;
use snafu::ResultExt;
use structopt::StructOpt;
use tonic::transport::Server;
use tracing::{subscriber, Level};
use tracing_fmt::FmtSubscriber;

mod accounts;
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

    let config = tonic_web::config().allow_all_origins();

    Server::builder()
        .accept_http1(true)
        .add_service(config.enable(NameServiceServer::new(service)))
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
