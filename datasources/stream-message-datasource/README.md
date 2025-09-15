# Carbon Stream Message Datasource

A generic and extensible datasource for the indexing framework that consumes a unified stream of account and transaction messages from any source.

## Overview

`StreamMessageDatasource` provides an abstract interface for feeding real-time or batch data into Carbon, decoupled from any specific transport or plugin implementation.

It is designed to handle incoming messages like:

```rust
pub enum UnifiedMessage {
    Account(AccountUpdate),
    Transaction(Box<TransactionUpdate>),
}
```

This enables integration with various upstreams such as:

- **Solana Geyser plugins** (Agave, custom)
- **Snapshot replay**
- **Kafka or PubSub streams**
- **Custom ingestors**

## Features

- Accepts both **account** and **transaction** updates
- Handles message streams via `handle_message_stream`
- Compatible with **cancellation tokens** and **metrics tracking**
- Integrates cleanly with Carbon's `Datasource` trait
- Uses Solana-native types via `solana-*` crates (`~2.3` compatible)

## Example Usage

```rust
use carbon_core::{datasource::Datasource, types::Update};
use carbon_stream_message_datasource::{StreamMessageDatasource, UnifiedMessage};

let (sender, receiver) = tokio::sync::mpsc::channel(100_000);
let datasource = StreamMessageDatasource::new(receiver);

pipeline.register(Box::new(datasource));
```

Send messages into the channel from any source:

```rust
sender.send(UnifiedMessage::Account(account_update)).await?;
sender.send(UnifiedMessage::Transaction(Box::new(tx_update))).await?;
```

## When to Use

Use `StreamMessageDatasource` when you:

- Want to stream blockchain data into Carbon in **real time**
- Have a **custom ingestion** source (e.g. WebSocket, snapshot, Kafka)
- Need to simulate on-chain activity using **historical replay**
- Want to prototype and test indexing pipelines **without writing a full datasource**
