# Carbon Jito Shredstream gRPC Datasource

_Shreds are fragments of data used in the Solana blockchain to represent parts of transactions before they are assembled into a full block. When a validator processes transactions, it breaks them down into smaller units called shreds. These shreds are distributed across the network..._ https://docs.jito.wtf/lowlatencytxnfeed/

> [!WARNING]  
> While Shredstream is great for real-time, low-latency data access, it **must not be used for indexing**: transactions are distributed as they are received by the leader, **without status or metadata**.
>
> For that purpose, it's recommended to use `block-subscribe` or `block-crawling` datasources instead.
