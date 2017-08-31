# nathanjent

Work in progress to create a framework for my personal web site using [Rust](rust-lang.org). I already pay to host some Wordpress sites on a LAMP server, I figured I could use it and finally learn how all this server stuff works.

### Technologies Used
- Rust
  - I guess you could call this a LAMR stack. So far I have created a CGI handler. At some point I might tackle FastCGI. I plan to add MySql database handling and some kind of HTML generation from a markup language, too. I took some inspiration and source code from [Rouille](https://github.com/tomaka/rouille) among other Rust web frameworks. 
- Vagrant
  - Run tests on a Debian LAMP stack based on [this](https://github.com/panique/vagrant-lamp-bootstrap)
- Make
  - Builds with Cargo and deploys to the linked web root in the VM
  - Run `vagrant ssh -c "make -C /vagrant"` to build within the VM
