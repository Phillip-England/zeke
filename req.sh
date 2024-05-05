#!/bin/bash

total_requests=100
concurrent_requests=10

for ((i = 1; i <= total_requests; i += concurrent_requests))
do
    # Start a batch of 10 concurrent requests
    for ((j = 0; j < concurrent_requests; j++))
    do
        curl localhost:8080/ &
    done

    # Wait for all concurrent requests to finish before starting the next batch
    wait
done
