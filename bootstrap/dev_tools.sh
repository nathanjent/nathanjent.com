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

# Install JavaScript development tools
curl -sL https://deb.nodesource.com/setup_8.x | sudo -E bash -
curl -sL https://dl.yarnpkg.com/debian/pubkey.gpg | sudo apt-key add -
echo "deb https://dl.yarnpkg.com/debian/ stable main" | sudo tee /etc/apt/sources.list.d/yarn.list
sudo apt-get update && sudo apt-get install -y yarn
