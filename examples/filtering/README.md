# Carbon Pipeline Example

This project demonstrates how to set up and run a Carbon pipeline and do filtering between datasources.

## Setup Instructions

### Step 1: Clone the Repository

To get started, clone the repository:

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd examples/pumpfun-activities
```

### Step 2: Set Environment Variables

Create a `.env` file in the root of your project and set the following environment variables:

```env
API_KEY=...
```

- `API_KEY` should point your Helius API key.

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

This will start the Geyser client and the pipeline will begin processing transactions for the specified program ID (`PUMPFUN_PROGRAM_ID`).

## Metrics

The example doesn't include a metrics implementation by default. However, you can easily integrate custom metrics or logging by passing your own metrics implementation to the pipeline.
