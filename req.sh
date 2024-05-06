#!/bin/bash

total_requests=100
concurrent_requests=10

start_time=$(date +%s)  # Capture start time in seconds

for ((i = 1; i <= total_requests; i += concurrent_requests))
do
    # Start a batch of concurrent requests
    for ((j = 0; j < concurrent_requests; j++))
    do
        curl localhost:8080 &
    done

    # Wait for all concurrent requests to finish before starting the next batch
    wait
done

end_time=$(date +%s)  # Capture end time in seconds
duration=$((end_time - start_time))  # Calculate duration

echo "Total process time: $duration seconds"
