# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.require_version ">= 1.8.0"
Vagrant.configure("2") do |config|
    config.vm.box = "debian/stretch64"

    config.vm.hostname = "dev.vagrant"
    config.vm.synced_folder "./", "/vagrant",
      disabled: true
    config.vm.synced_folder "./www/", "/var/www/vhosts/local",
      disabled: true
    config.vm.synced_folder "./static/", "/var/www/vhosts/local",
      disabled: false

    config.vm.network "forwarded_port", guest: 8080, host: 8080

    config.vm.provision "ansible" do |a|
      a.verbose = "v"
      a.playbook = "./playbook.yml"
      a.galaxy_role_file = "./requirements.yml"
    end
end
