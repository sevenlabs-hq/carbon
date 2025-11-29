# Jetstreamer Example

This examples demonstrates how to use [Jetstreamer](https://github.com/anza-xyz/jetstreamer) as a datasource for a Carbon pipeline.

## Setup Instructions

### Step 1: Clone the Repository

To get started, clone the repository:

```sh
git clone git@github.com:sevenlabs-hq/carbon.git
cd examples/jetstreamer
```

### Step 2: Build the Project

To compile the project, run the following command:

```sh
cargo build --release
```

### Step 3: Run the Pipeline

After building the project, you can run the pipeline using:

```sh
RUST_LOG=info cargo run --release
```

This will start the Jetstreamer client and the pipeline will begin processing transactions. In our example, we're filtering in only Token 2022 program transactions.