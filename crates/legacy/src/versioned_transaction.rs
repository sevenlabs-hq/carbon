use legacy_solana_transaction::versioned::VersionedTransaction;

pub fn to_modern(
    legacy: VersionedTransaction,
) -> solana_transaction::versioned::VersionedTransaction {
    let bytes = bincode::serialize(&legacy).expect("serialize v2 versioned transaction");
    bincode::deserialize::<solana_transaction::versioned::VersionedTransaction>(&bytes)
        .expect("deserialize v3 versioned transaction")
}
