fn main() {
    println!("cargo:rerun-if-changed=./src/signer.udl");
    uniffi_build::generate_scaffolding("./src/signer.udl").unwrap();
}
