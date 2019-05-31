#!/usr/bin/env bash
# bootstrap.sh
# Runs commands needed to get started with development

echo 'Ensuring Rust nightly toolchain selected...'
if [[ ! $(rustc -V) =~ "nightly" ]]; then
  echo 'Please switch your Rust toolchain to nightly -- conisder using Rustup!'
  echo 'With rustup installed: rustup override set nightly'
  exit 1
fi
echo 'Done!'

echo 'Initializing Cargo dependencies and checking repo source code...'
cargo check
if [[ $? != 0 ]]; then
  echo 'Cargo check failed!'
  echo 'Please comb the output to find out why <3'
  exit 1
fi
echo 'Done!'

echo 'Verifying SQLite3 installed...'
command -v sqlite3 >/dev/null 2>&1
if [[ $? != 0 ]]; then
  echo 'sqlite3 CLI missing -- please install it using your package manager!'
  echo 'We also need the devel libraries, so install those too! <3'
  exit 1
fi
echo 'Done!'

echo 'Installing Diesel CLI if it is missing...'
command -v diesel >/dev/null 2>&1 ||
cargo install diesel_cli --no-default-features --features sqlite
if [[ $? != 0 ]]; then
  echo 'Diesel CLI install failed!'
  echo 'Please comb the output to find out why <3'
  exit 1
fi
echo 'Done!'

echo 'Setting up local dev SQLite DB if missing...'
DB_PATH="${PWD}/db/splat_challenges.db"
if [[ ! -f "$DB_PATH" ]]; then
  diesel --database-url "${DB_PATH}" setup
  if [[ $? != 0 ]]; then
    echo 'Dev DB setup failed!'
    echo 'Please comb the output to find out why <3'
    exit 1
  fi
else
  echo 'Skipping setup as it seems the DB aleardy exists...'
fi
echo 'Done!'

echo 'Verifiying tests are runable...'
cargo test
if [[ $? != 0 ]]; then
  echo 'There was some failure in the tests!'
  echo 'Please comb the output to find out why <3'
  exit 1
fi
echo 'Done!'

echo 'All good to start developing! <3'
