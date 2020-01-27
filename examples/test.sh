#!/bin/bash

for i in $(seq 1 5); do
    echo -n "[Iteration $i] "
    for j in $(seq 1 20); do
        echo -n .
        sleep 0.05
    done
    echo
done
