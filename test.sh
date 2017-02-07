#!/bin/bash
cd ~/Documents/programmieren/c/snowball && make snowball
cd ~/Documents/programmieren/rust/snowball-rs/
for i in algorithms/*.sbl; do
  filename=$(basename "$i")  
  ~/Documents/programmieren/c/snowball/snowball "$i" -rust -o "src/snowball/algorithms/${filename%.*}"
done         
cargo test

