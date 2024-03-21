ulimit -n 1000000
tmutil thinlocalsnapshots /
# If not enough then
# tmutil thinlocalsnapshots / 500000000000 4
cargo run -r
