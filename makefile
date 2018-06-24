DEBUG_DIR =  target/debug
RELEASE_DIR = target/release
DEPLOY_DIR = www/nathanjent
WASM_DIR =  target/wasm32-unknown-unknown/release

.PHONY: all debug release doc test clean

all: release

debug: debugcgi bundleclient

release: releasecgi bundleclient

clean: cleancgi cleanclient

debugcgi:
	cd backend ; \
		cargo build ; 
	cp -u backend/$(DEBUG_DIR)/nathanjent $(DEPLOY_DIR)/index.cgi ;
	cp -u static/.htaccess $(DEPLOY_DIR)/ ;
	cp -u static/.env $(DEPLOY_DIR)/

releasecgi:
	cd backend ; \
		cargo build --release ;
	cp -u $(RELEASE_DIR)/nathanjent $(DEPLOY_DIR)/index.cgi ; 
	cp -u static/.htaccess $(DEPLOY_DIR)/ ;
	cp -u static/.env $(DEPLOY_DIR)/

bundleclient:
	cd client ; \
		parcel build --out-dir ../$(DEPLOY_DIR) --public-url . src/index.html

doc:
	cargo doc

test:
	cargo test

cleancgi:
	cd backend ; \
		cargo clean
	rm -f $(DEPLOY_DIR)/index.cgi

cleanclient:
	rm -rf $(DEPLOY_DIR)/*

vagrant:
	vagrant ssh -c "make -C /vagrant"

vbuild:
	vagrant ssh -c "make -C /vagrant debug"

vrelease:
	vagrant ssh -c "make -C /vagrant release"

vdoc:
	vagrant ssh -c "make -C /vagrant doc"

vtest:
	vagrant ssh -c "make -C /vagrant test"

vclean:
	vagrant ssh -c "make -C /vagrant clean"

