fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=dns.proto");

    tonic_build::compile_protos("./dns.proto").expect("failed to compile protobuf");
}
