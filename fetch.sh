#!/bin/bash

if [ -z "$1" ]; then
  echo "Provide a day number."
  echo "usage: $0 DAY"
  exit 1
fi

if [ -z "$AOC_TOKEN" ]; then
  echo "No session token."
  exit 1
fi

URL="https://adventofcode.com/2021/day/$1/input"
cargo new day$1
cd day$1
curl $URL --cookie $AOC_TOKEN > input.txt
touch sample.txt
cp ../template.rs src/main.rs
echo "itertools = \"0.8.0\"" >> Cargo.toml