# Carbon Stream Message Datasource

A generic and extensible datasource for the indexing framework that consumes a unified stream of account and transaction messages from any source.

## Overview

`StreamMessageClient` provides an abstract interface for feeding real-time or batch data into Carbon, decoupled from any specific transport or plugin implementation.

It is designed to handle incoming messages like:

```rust,ignore
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

Give the receiving end of a channel to `StreamMessageClient`, add the client to a pipeline, and send updates on the other end. Here a task replays a list of updates:

```rust
use {
    carbon_core::{error::CarbonResult, pipeline::Pipeline},
    carbon_stream_message_datasource::{StreamMessageClient, UnifiedMessage},
    tokio::sync::mpsc,
};

async fn index(updates: Vec<UnifiedMessage>) -> CarbonResult<()> {
    let (sender, receiver) = mpsc::channel(100_000);

    tokio::spawn(async move {
        for update in updates {
            if sender.send(update).await.is_err() {
                break; // the pipeline has stopped
            }
        }
    });

    let mut pipeline = Pipeline::builder()
        .datasource(StreamMessageClient::new(receiver))
        // Register your pipes here, e.g. `.account(MyDecoder, MyProcessor)`.
        .build()?;

    pipeline.run().await
}
```

From a live source, send each update as it arrives:

```rust
use {
    carbon_core::datasource::{AccountUpdate, TransactionUpdate},
    carbon_stream_message_datasource::UnifiedMessage,
    tokio::sync::mpsc::{error::SendError, Sender},
};

async fn forward(
    sender: &Sender<UnifiedMessage>,
    account_update: AccountUpdate,
    transaction_update: TransactionUpdate,
) -> Result<(), SendError<UnifiedMessage>> {
    sender.send(UnifiedMessage::Account(account_update)).await?;
    sender
        .send(UnifiedMessage::Transaction(Box::new(transaction_update)))
        .await?;
    Ok(())
}
```

## When to Use

Use `StreamMessageClient` when you:

- Want to stream blockchain data into Carbon in **real time**
- Have a **custom ingestion** source (e.g. WebSocket, snapshot, Kafka)
- Need to simulate on-chain activity using **historical replay**
- Want to prototype and test indexing pipelines **without writing a full datasource**
