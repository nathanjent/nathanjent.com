# -*- mode: ruby -*-
# vi: set ft=ruby :

# Load dotenv
begin
    require 'dotenv'
    Dotenv.load
rescue LoadError => e
    $stderr.puts "could not load .env file"
end

# Pass the config version
Vagrant.configure("2") do |config|

    # Every Vagrant virtual environment requires a box to build off of.
    config.vm.box = "debian/jessie64"

    # Create a private network, which allows host-only access to the machine using a specific IP.
    #config.vm.network "private_network", ip: "192.168.33.22"
    config.vm.network "forwarded_port", guest: 80, host: 8080, host_ip: "127.0.0.1"

    # Share an additional folder to the guest VM. The first argument is the path on the host to the actual folder.
    # The second argument is the path on the guest to mount the folder.
    config.vm.synced_folder "./", "/vagrant", type: "rsync"
    config.vm.synced_folder "./www/", "/var/www", type: "rsync"

    # Define the bootstrap file: A (shell) script that runs after first setup of your box (= provisioning)
    config.vm.provision :shell, path: "bootstrap.sh"

    # Define a push strategy for nathanjent.com
    config.push.define "staging", strategy: "sftp" do |push|
        # Push to my site ftp
        push.host = ENV["FTP_HOST"]
        push.username = ENV["FTP_USERNAME"]
    end

    config.vm.provision "shell", privileged: false, inline: <<-RUST
        # install rust compiler
        curl https://sh.rustup.rs -sSf | sh -s -- -y
    RUST

    config.vm.provision "shell", privileged: false, inline: <<-RUST
        # install diesel and run DB migrations
        cargo install diesel_cli --no-default-features --features mysql
        cd /vagrant
        diesel migration run
    RUST
end
