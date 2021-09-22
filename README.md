# Rust Benchmarking Example Code

This repository contains the sample code included in the [Making Slow Rust Code Fast]() blog post.

## Setup

1. Clone the `mongo-rust-driver` repository to the parent directory:

``` shell
cd .. && git clone https://github.com/mongodb/mongo-rust-driver
```

2. Start a mongodb instance

``` shell
mongod --dbpath <some folder> --logpath <some log file> --fork
```

## Running the benchmark

`cargo bench` will run the `find` benchmark. `target/criterion/report` will contain the HTML report.

The code for the benchmark is found in `benches/find.rs`.

## Generating a flamegraph

`cargo flamegraph -o <filename.svg> --bin benchmark-example` will generate a low-noise flamegraph. Be sure to seed the database with data before generating the flamegraph (see the `see_data` function).

The code for the application to use for flamegraphs can be found in `src/main.rs`.
