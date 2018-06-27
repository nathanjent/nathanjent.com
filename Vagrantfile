# -*- mode: ruby -*-
# vi: set ft=ruby :

# Load dotenv to set custom environment variables
begin
    require 'dotenv'
    Dotenv.load
rescue LoadError => e
    $stderr.puts "could not load .env file error: #{e}"
end

# Pass the config version
Vagrant.configure("2") do |config|

    # Every Vagrant virtual environment requires a box to build off of.
    config.vm.box = "debian/jessie64"

    # Create a private network, which allows host-only access to the machine using a specific IP.
    #config.vm.network "private_network", ip: "192.168.33.22"
    config.vm.network "forwarded_port", guest: 80, host: 8080, host_ip: "127.0.0.1"

    # Set the hostname
    config.vm.hostname = "dev-jessie64.vagrant"

    # Enable agent forwarding via SSH
    config.ssh.forward_agent = true

    # Share an additional folder to the guest VM. The first argument is the path on the host to the actual folder.
    # The second argument is the path on the guest to mount the folder.
    config.vm.synced_folder "./", "/vagrant",
    #    type: "rsync"
        type: "nfs",
        nfs_udp: false,
        nfs_version: 4

    config.vm.synced_folder "./www/" + ENV["SITE_NAME"], "/var/www/" + ENV["SITE_NAME"],
    #    type: "rsync"
        type: "nfs",
        nfs_udp: false,
        nfs_version: 4

    # Provision the basics 
    config.vm.provision :shell,
        path: "bootstrap/dev_tools.sh"
    
    # Provision apache
    config.vm.provision :shell,
        path: "bootstrap/apache.sh",
        env: { "SITE_NAME" => ENV["SITE_NAME"] }

    # Provision MySQL
    config.vm.provision :shell,
        path: "bootstrap/mysql.sh",
        env: {
            "DBROOT_PASS" => ENV["DATABASE_ROOT_PASS"],
            "DBPASS" => ENV["DATABASE_PASS"],
            "DBUSER" => ENV["DATABASE_USER"],
        }

    # Provision PHM MyAdmin
    config.vm.provision :shell,
        path: "bootstrap/phpmyadmin.sh",
        env: {
            "PHPPASS" => ENV["DATABASE_ROOT_PASS"],
        }

    # Provision Rust dev tools
    config.vm.provision "shell",
        privileged: false,
        path: "bootstrap/rust.sh"

    # Setup Diesel and run the migrations on the DB in the VM
    config.vm.provision "shell",
        privileged: false,
        path: "bootstrap/diesel.sh"

    # Define a push strategy for nathanjent.com
    config.push.define "staging", strategy: "sftp" do |push|
        # Push to my site ftp
        push.host = ENV["FTP_HOST"]
        push.username = ENV["FTP_USERNAME"]
    end
end
