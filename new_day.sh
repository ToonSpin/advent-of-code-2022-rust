#!/bin/bash

if [[ $# -ne 1 ]]; then
    echo "Usage: `basename $0` <DAY_NUMBER>"
    exit 1
fi

filename_src=$(printf "./src/bin/day%02d.rs" $1)
filename_data=$(printf "./data/day%02d.txt" $1)

cat << RUST > $filename_src
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    Ok(())
}
RUST

touch $filename_data
