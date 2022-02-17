#!/usr/bin/env bash

# Install mysql and give password to installer
sudo debconf-set-selections <<< "mysql-server mysql-server/root_password password $DBROOT_PASS"
sudo debconf-set-selections <<< "mysql-server mysql-server/root_password_again password $DBROOT_PASS"
sudo apt-get -y install mysql-server

# Setup mysql test database and user
DBSETUP=$(cat <<EOF
CREATE DATABASE IF NOT EXISTS test;
GRANT ALL ON test.* TO '${DBUSER}'@'localhost' IDENTIFIED BY '${DBPASS}';
EOF
)
echo "${DBSETUP}" > /vagrant/db_setup.sql
mysql -uroot -p"${DBROOT_PASS}" < /vagrant/db_setup.sql
rm /vagrant/db_setup.sql
