#!/bin/bash
./setup.bash
cargo build --release
set e
cd examples
for file in $(find -name "*.rs"); do 
  echo ""
  echo ""
  echo "----------------------"
  echo "Running example $file"; 
  echo "-----------------------"
  echo ""
  echo ""
  ../target/release/rnr "$file" --type-check --vm -m 10 --target "mips" -o "./$file.asm" 
done;
