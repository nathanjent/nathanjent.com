#!/usr/bin/env bash

# Update / upgrade
sudo apt-get update
sudo apt-get -y upgrade

# Install curl to get rustup
sudo apt-get -y install curl

# Install other development tools to compile C libraries
sudo apt-get install -y build-essential

# Install dev libs for diesel
sudo apt-get install -y libmysqlclient-dev
