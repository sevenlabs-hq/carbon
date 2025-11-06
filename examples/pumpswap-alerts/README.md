# PumpSwap LaserStream Alerts - Carbon Pipeline Example

This project demonstrates how to set up and run a Carbon pipeline using **Helius LaserStream** as the data source to monitor PumpSwap protocol activities on Solana. Unlike traditional Yellowstone gRPC implementations, LaserStream provides enhanced reliability with automatic reconnection, slot replay capabilities, and never-miss-data guarantees.

## Setup Instructions

### Step 1: Clone the Repository

```sh
git clone https://github.com/sevenlabs-hq/carbon.git
cd examples/pumpswap-alerts
```

### Step 2: Set Environment Variables

Create a `.env` file in the root of your project and set the following environment variables:

```env
LASERSTREAM_URL=...
API_KEY=...
```

- `LASERSTREAM_URL` should point your Helius LaserStream endpoint.
- `API_KEY` should point your Helius API key.

**Available LaserStream Endpoints:**

- `https://laserstream-mainnet-ewr.helius-rpc.com` (US East - Newark)
- `https://laserstream-mainnet-pitt.helius-rpc.com` (US Central - Pittsburgh)
- `https://laserstream-mainnet-slc.helius-rpc.com` (US West - Salt Lake City)
- `https://laserstream-mainnet-lax.helius-rpc.com` (US West Coast - Los Angeles)
- `https://laserstream-mainnet-ams.helius-rpc.com` (Europe - Amsterdam)
- `https://laserstream-mainnet-fra.helius-rpc.com` (Europe - Frankfurt)
- `https://laserstream-mainnet-tyo.helius-rpc.com` (Asia - Tokyo)
- `https://laserstream-mainnet-sgp.helius-rpc.com` (Asia - Singapore)

### Step 3: Build the Project

```sh
cargo build --release
```

### Step 4: Run the Pipeline

After building the project, you can run the pipeline using:

```sh
cargo run --release
```

This will start the Geyser client and the pipeline will begin processing transactions for the specified program ID (`PUMPSWAP_PROGRAM_ID`).

## Metrics

The example doesn't include a metrics implementation by default. However, you can easily integrate custom metrics or logging by passing your own metrics implementation to the pipeline.
