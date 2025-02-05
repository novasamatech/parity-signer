use crate::MetadataProof;
use codec::Decode;
use hex;

#[test]
fn parse_proof() {
    let data = hex::decode("a40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33").unwrap();
    let metadata = MetadataProof::decode(&mut &data[..]).ok().unwrap();
    println!("{:?}", metadata);
}