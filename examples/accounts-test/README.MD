# Account Pipe Example

This example demonstrates how a real-world application might utilize Carbon to index specific accounts in a database, using Geyser. It focuses on the `Account` and `AccountDeletion` pipes.

## Running the Example

To set up the database for this example, use the Diesel CLI. Once in the `accounts-test` example directory, you can run migrations on a newly created database with:

```bash
diesel migration run --database-url INSERT_DATABASE_CONNECTION_URL
```

In `main.rs`, update the `GRPC` variable with a valid endpoint and x-token. Then, run the app with:

```bash
cargo run -p accounts-test
```

## Database

For simplicity, we use the PumpFun program, which has two relevant accounts: `Bonding Curve` and `Global Account`. Using Diesel, we created the schema, migrations, and models necessary for the service.

```rust
diesel::table! {
    bonding_curve (id) {
        id -> Int4,
        pubkey -> VarChar,
        virtual_token_reserves -> Int8,
        virtual_sol_reserves -> Int8,
        real_token_reserves -> Int8,
        real_sol_reserves -> Int8,
        token_total_supply -> Int8,
        complete -> Bool,
    }
}

diesel::table! {
    global_account (id) {
        id -> Int4,
        pubkey -> VarChar,
        initialized -> Bool,
        #[max_length = 44]
        authority -> Varchar,
        #[max_length = 44]
        fee_recipient -> Varchar,
        initial_virtual_token_reserves -> Int8,
        initial_virtual_sol_reserves -> Int8,
        initial_real_token_reserves -> Int8,
        token_total_supply -> Int8,
        fee_basis_points -> Int8,
    }
}
```

See the `db` directory in this crate for more details.

## Tracked Accounts

To efficiently keep track of accounts, we store the account public keys in memory, avoiding unnecessary database queries. Here’s an implementation that works well:

```rust
let mut bonding_curve_pubkeys = diesel::query_dsl::methods::FilterDsl::filter(
    crate::db::schema::bonding_curve::dsl::bonding_curve,
    crate::db::schema::bonding_curve::dsl::id.is_not_null(),
)
.select(crate::db::schema::bonding_curve::dsl::pubkey)
.load::<String>(&mut db::get_conn())
.expect("Failed to load tracked wallets")
.into_iter()
.collect::<HashSet<String>>();
log::trace!("Fetched wallets that are tracked...");

let global_account_pubkeys = diesel::query_dsl::methods::FilterDsl::filter(
    crate::db::schema::global_account::dsl::global_account,
    crate::db::schema::global_account::dsl::id.is_not_null(),
)
.select(crate::db::schema::global_account::dsl::pubkey)
.load::<String>(&mut db::get_conn())
.expect("Failed to load tracked wallets")
.into_iter()
.collect::<HashSet<String>>();
log::trace!("Fetched wallets that are tracked...");

bonding_curve_pubkeys.extend(global_account_pubkeys);

let account_deletions = bonding_curve_pubkeys
    .iter()
    .map(|str| Pubkey::from_str(str).unwrap())
    .collect::<HashSet<Pubkey>>();

let tracked_accounts = Arc::new(RwLock::new(account_deletions));
```

This allows us to efficiently supply the necessary accounts to our processors, in form of a hashset.

## Geyser GRPC

To filter transactions specific to the PumpFun program, use the premade Geyser data source from the `carbon-core` crate:

```rust
let account_filters = HashMap::new();

let mut transaction_filters = HashMap::new();
transaction_filters.insert(
    "pumpfun".to_string(),
    SubscribeRequestFilterTransactions {
        vote: None,
        failed: None,
        signature: None,
        account_include: vec![],
        account_exclude: vec![],
        account_required: vec!["6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P".to_owned()],
    },
);
```

Here we choose to only get transactions from the PumpFun program.
Now that we have filters, we can create the Geyser data source:

```rust
let grpc = YellowstoneGrpcGeyserClient {
    commitment: Some(CommitmentLevel::Processed),
    account_filters,
    transaction_filters,
    account_deletions_tracked: tracked_accounts.clone(),
    endpoint: "ENDPOINT".to_string(),
    x_token: Some("X-TOKEN".to_string()),
};
```

## Decoder

To continue setting up the pipeline, we need a Decoder for our program. Since it’s an Anchor program, use the Carbon CLI to generate the decoder:

```bash
cargo run parse --idl pf_idl.json --output ../../examples/accounts-test/src/
```
