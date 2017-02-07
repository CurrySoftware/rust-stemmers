#!/bin/bash
for i in algorithms/*.sbl; do
  filename=$(basename "$i")  
  snowball "$i" -rust -o "src/snowball/algorithms/${filename%.*}"
done         
cargo test

