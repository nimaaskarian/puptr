#!/bin/bash
cargo build --release
CMDS=("./target/release/puptr < tests/ok.html"
  "./target/release/puptr < tests/error.html"
  "./target/release/puptr p < tests/ok.html"
  "./target/release/puptr a < tests/ok.html"
  "./target/release/puptr head < tests/ok.html"
  "./target/release/puptr body < tests/ok.html"
  "./target/release/puptr .paragraph < tests/ok.html"
  "./target/release/puptr '#special-divider' < tests/ok.html"
)
for cmd in "${CMDS[@]}";do
  echo $cmd
  bash -c "$cmd"
  printf "\n\n"
done
