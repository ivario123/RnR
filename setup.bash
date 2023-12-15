#!/bin/bash

mkdir -p testing
ls -la
cp -r ./* ./testing/
git clone https://vesuvio-git.neteq.ltu.se/pln/mips mips
cd testing
