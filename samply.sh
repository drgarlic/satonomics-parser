ulimit -n 100000
cargo build --profile profiling && samply record ./target/profiling/explorer
