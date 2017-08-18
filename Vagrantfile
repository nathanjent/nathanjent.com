# -*- mode: ruby -*-
# vi: set ft=ruby :

# Vagrantfile API/syntax version. Don't touch unless you know what you're doing!
VAGRANTFILE_API_VERSION = "2"

Vagrant.configure(VAGRANTFILE_API_VERSION) do |config|

  # Every Vagrant virtual environment requires a box to build off of.
  config.vm.box = "debian/contrib-jessie64"

  # Create a private network, which allows host-only access to the machine using a specific IP.
  #config.vm.network "private_network", ip: "192.168.33.22"
  config.vm.network "forwarded_port", guest: 80, host: 8080, host_ip: "127.0.0.1"

  # Share an additional folder to the guest VM. The first argument is the path on the host to the actual folder.
  # The second argument is the path on the guest to mount the folder.
  config.vm.synced_folder "www/", "/var/www/"

  # Define the bootstrap file: A (shell) script that runs after first setup of your box (= provisioning)
  config.vm.provision :shell, path: "bootstrap.sh"

  # Define a push strategy for nathanjent.com
  config.push.define "staging", strategy: "sftp" do |push|
      # TODO
      push.host = ""
      push.username = ""
  end

  config.vm.provision "shell", privileged: false, inline: <<-RUST
    # install rust compiler
    sudo apt-get -y install curl
    curl https://sh.rustup.rs -sSf | sh -s -- -y

    # install other development tools to compile C libraries
    sudo apt-get install -y build-essential
  RUST

end
