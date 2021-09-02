fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=at2-ns.proto");

    tonic_build::compile_protos("./at2-ns.proto").expect("failed to compile protobuf");
}
