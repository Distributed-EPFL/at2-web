use std::io;

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=at2-ns.proto");

    let mut conf = tonic_build::configure();
    conf = conf.build_client(false).build_server(false);

    #[cfg(feature = "client")]
    {
        conf = conf.build_client(true);
    }

    #[cfg(feature = "server")]
    {
        conf = conf.build_server(true);
    }

    conf.compile(&["./at2-ns.proto"], &["."])
}
