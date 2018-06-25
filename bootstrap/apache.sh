#!/usr/bin/env bash

# Create project folder
PROJECTPATH="/var/www/${SITE_NAME}/"
if [[ ! -d "${PROJECTPATH}" ]]; then
    sudo mkdir -p "${PROJECTPATH}"
fi

# Install apache 2.5 and php 5.5
sudo apt-get install -y apache2

# Setup hosts file for Apache server
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

# Enable mod_rewrite
sudo a2enmod rewrite

# Enable mod_cgi
sudo a2enmod cgi

# Restart Apache
sudo service apache2 restart
