# Carbon Block Crawler Example

This project demonstrates how to set up and run a Carbon pipeline that processes Solana transactions using the Pumpfun decoder. It utilizes the `RpcBlockCrawler` as a data source to fetch Solana transactions and events. The example showcases how to decode Pumpfun instructions and implement a custom processor to handle specific events.

## Setup Instructions

### Step 1: Clone the Repository

To get started, clone the repository:

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd examples/block-crawler
```

### Step 2: Set Environment Variables

Create a `.env` file in the root of your project and set the following environment variable:

```env
RPC_URL=your_rpc_url
```

- Replace `your_rpc_url` with the actual RPC URL you intend to use.

### Step 3: Build the Project

To compile the project, run the following command:

```sh
cargo build --release
```

### Step 4: Run the Pipeline

After building the project, you can run the pipeline using:

```sh
cargo run --release -- --start-slot <start_slot> --end-slot <end_slot>
```

Replace `<start_slot>` and `<end_slot>` with the desired Solana slot range for processing.
