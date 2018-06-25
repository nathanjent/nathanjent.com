#!/usr/bin/env bash

# Install diesel and run DB migrations
cargo install diesel_cli --no-default-features --features mysql
cd /vagrant
diesel migration run
