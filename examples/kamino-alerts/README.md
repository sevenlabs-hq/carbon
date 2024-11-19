# Carbon Pipeline Example

This project demonstrates how to set up and run a Carbon pipeline that processes Solana transactions using the Kamino Lending decoder. It utilizes the `YellowstoneGrpcGeyserClient` as a data source to fetch Solana transactions and events. The example showcases how to decode Kamino Lending instructions and implement a custom processor to handle specific events.

## Setup Instructions

### Step 1: Clone the Repository

To get started, clone the repository:

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd examples/kamino-alerts
```

### Step 2: Set Environment Variables

Create a `.env` file in the root of your project and set the following environment variables:

```env
GEYSER_URL=...
X_TOKEN=...
```

- `GEYSER_URL` should point to the Yellowstone Geyser RPC URL you want to use for Solana transaction crawling.
- `X_TOKEN` is optional and can be used if your Geyser endpoint requires an authentication token.

### Step 3: Build the Project

To compile the project, run the following command:

```sh
cargo build --release
```

### Step 4: Run the Pipeline

After building the project, you can run the pipeline using:

```sh
cargo run --release
```

This will start the Geyser client and the pipeline will begin processing transactions for the specified program ID (`KAMINO_LENDING_PROGRAM_ID`).

## Metrics

The example doesn't include a metrics implementation by default. However, you can easily integrate custom metrics or logging by passing your own metrics implementation to the pipeline.
