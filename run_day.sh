#!/bin/bash

if [[ $# -ne 1 ]]; then
    echo "Usage: `basename $0` <DAY_NUMBER>"
    exit 1
fi

filename_program=$(printf "./target/release/day%02d" $1)
filename_data=$(printf "./data/day%02d.txt" $1)

$filename_program < $filename_data
