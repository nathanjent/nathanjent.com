CARGO = cargo
RELEASE_DIR = target/release
DEBUG_DIR =  target/debug
DEPLOY_DIR = www/nathanjent

build:
	@$(CARGO) build
	cp -f $(DEBUG_DIR)/nathanjent $(DEPLOY_DIR)/index.cgi
	cp -f static/.htaccess $(DEPLOY_DIR)/
	cp -f static/.env $(DEPLOY_DIR)/

release:
	@$(CARGO) build --release
	cp -f $(RELEASE_DIR)/nathanjent $(DEPLOY_DIR)/index.cgi

doc:
	@$(CARGO) doc

check: build test

test:
	@$(CARGO) test

bench:
	@$(CARGO) bench

clean:
	@$(CARGO) clean
	rm -rf $(DEPLOY_DIR)/*

# Call this outside of VM only
vagrant:
	vagrant ssh -c 'make -C /vagrant'

.PHONY: all build doc check test bench clean
