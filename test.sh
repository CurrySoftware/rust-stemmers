#!/bin/bash
cd ~/Documents/programmieren/c/snowball && make snowball
cd ~/Documents/programmieren/rust/snowball-rs/src/
for i in *.sbl; do
  filename=$(basename "$i")  
  ~/Documents/programmieren/c/snowball/snowball "$i" -rust -o "${filename%.*}"
done         
cargo test

