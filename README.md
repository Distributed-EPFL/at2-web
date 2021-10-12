# at2-web

[![at2-web](https://github.com/Distributed-EPFL/at2-web/actions/workflows/rust.yml/badge.svg)](https://github.com/Distributed-EPFL/at2-web/actions/workflows/rust.yml)

A demonstrator for [at2-node](https://github.com/Distributed-EPFL/at2-node).
It allows for creating an account on the nodes running at
[EPFL](https://www.epfl.ch).
You can then send some assets with it and even test the speed of the network.
Overall, it both shows and explains the capability of AT2.

## crates

There are two crates, one for the demo itself and one for the dns, both in rust.

### web

```sh
# install trunk, the web framework CLI
cargo install trunk

# you can serve it locally
trunk serve
xdg-open localhost:8080

# or you can build it and serve it via a real webserver
trunk build --release
```

### at2-ns

The demonstrator needs a way to map users' public keys to human readable names,
it needs a name service. As the network is already existing, the demo uses
EPFL's one. But if you feel adventurous, you can spin your own name service.

```sh
cargo run --features server 127.0.0.1:1234
```

You can then modify `web/src/config.rs` to point to your local service.
