#!/bin/bash

for i in $(seq -w 26); do
    if [ -x "target/release/day${i}" ]; then
        echo "------------------------------------------------------------------------- DAY ${i}"
        "target/release/day${i}" < "data/day${i}.txt"
    fi
done
