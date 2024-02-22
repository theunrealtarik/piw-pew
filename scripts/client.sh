#!/bin/bash

# didn't use chat gpt btw

if [ $# -ne 1 ]; then
    echo "Usage: $0 <num_instances>"
    exit 1
fi

num_instances=$1

pids=()

for (( i=1; i<=$num_instances; i++ )); do
    echo "Starting client instance $i"
     RUST_BACKTRACE=1 cargo run --bin client &
    pids+=($!)
done

for pid in "${pids[@]}"; do
    wait $pid
done

echo "All client instances have finished."
