# Pumpfun Program Subscribe Example

This example demonstrates how to use the RPC Program Subscribe datasource to monitor Pumpfun program instructions in real-time.

## Features

- Uses `RpcProgramSubscribe` datasource to subscribe to Pumpfun program account changes
- Processes Pumpfun instructions including Create, Buy, Sell, and CPI events
- Logs detailed information about token creation, trades, and significant events
- Highlights large trades (>10 SOL)

## Setup

1. Copy the environment file:
   ```bash
   cp .env.example .env
   ```

2. Update `.env` with your WebSocket RPC URL:
   ```
   RPC_WS_URL=wss://your-websocket-rpc-url-here
   ```

## Running

```bash
cargo run --bin pumpfun-program-subscribe-carbon-example
```

## What it does

The example subscribes to the Pumpfun program and processes all instructions, logging:

- **Token Creation**: New tokens created with name, symbol, URI, and mint address
- **Buy Orders**: Token purchases with amounts and user information
- **Sell Orders**: Token sales with amounts and minimum SOL output
- **CPI Events**: Create events, trade events (highlighting large trades), and completion events

This provides real-time monitoring of Pumpfun activity using the RPC program subscribe mechanism.