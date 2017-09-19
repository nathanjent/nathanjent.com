#!/usr/bin/env bash

# Use single quotes instead of double quotes to make it work with special-character passwords
PASSWORD='12345678'
PROJECT='nathanjent'

# create project folder
PROJECTPATH="/var/www/${PROJECT}/"
if [[ ! -d "${PROJECTPATH}" ]]; then
    sudo mkdir -p "${PROJECTPATH}"
fi

# update / upgrade
sudo apt-get update
sudo apt-get -y upgrade

# install curl to get rustup
sudo apt-get -y install curl

# install other development tools to compile C libraries
sudo apt-get install -y build-essential
sudo apt-get install -y libmysqlclient-dev

# install apache 2.5 and php 5.5
sudo apt-get install -y apache2
sudo apt-get install -y php5

# install mysql and give password to installer
sudo debconf-set-selections <<< "mysql-server mysql-server/root_password password $PASSWORD"
sudo debconf-set-selections <<< "mysql-server mysql-server/root_password_again password $PASSWORD"
sudo apt-get -y install mysql-server
sudo apt-get install php5-mysql

# install phpmyadmin and give password(s) to installer
# for simplicity I'm using the same password for mysql and phpmyadmin
sudo debconf-set-selections <<< "phpmyadmin phpmyadmin/dbconfig-install boolean true"
sudo debconf-set-selections <<< "phpmyadmin phpmyadmin/app-password-confirm password $PASSWORD"
sudo debconf-set-selections <<< "phpmyadmin phpmyadmin/mysql/admin-pass password $PASSWORD"
sudo debconf-set-selections <<< "phpmyadmin phpmyadmin/mysql/app-pass password $PASSWORD"
sudo debconf-set-selections <<< "phpmyadmin phpmyadmin/reconfigure-webserver multiselect apache2"
sudo apt-get -y install phpmyadmin

# setup hosts file for Apache server
VHOST=$(cat <<EOF
<VirtualHost *:80>
    DocumentRoot "${PROJECTPATH}"
    <Directory "${PROJECTPATH}">
        AllowOverride All
        Require all granted
        Options +ExecCGI -MultiViews +SymLinksIfOwnerMatch
        AddHandler cgi-script .cgi
    </Directory>
</VirtualHost>
EOF
)
echo "${VHOST}" > /etc/apache2/sites-available/000-default.conf

rm -rf /var/www/html

# enable mod_rewrite
sudo a2enmod rewrite

# enable mod_cgi
sudo a2enmod cgi

# restart Apache
sudo service apache2 restart

# setup mysql test database and user
DBSETUP=$(cat <<EOF
CREATE DATABASE IF NOT EXISTS test;
GRANT ALL ON test.* TO 'nathanjent'@'localhost' IDENTIFIED BY 'firetruck';
EOF
)
echo "${DBSETUP}" > /vagrant/db_setup.sql
mysql -uroot -p"${PASSWORD}" < /vagrant/db_setup.sql

# the username and password should match the DATABASE_URL variable in .env for diesel
echo DATABASE_URL=mysql://nathanjent:firetruck@localhost:3306/test > /vagrant/.env
