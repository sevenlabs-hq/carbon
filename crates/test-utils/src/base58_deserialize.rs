pub fn ix_data(ix: &str) -> Vec<u8> {
    bs58::decode(ix).into_vec().expect("hex decode failed")
}
