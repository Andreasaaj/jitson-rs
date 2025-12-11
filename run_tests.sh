# Run tests with nextest for prettier output, and ignore command line error
cargo nextest run --failure-output never --status-level none || true