BACKEND_DIR = backend
DEBUG_DIR =  $(BACKEND_DIR)/target/debug
RELEASE_DIR = $(BACKEND_DIR)/target/release
DEPLOY_DIR = www/nathanjent
WASM_DIR =  target/wasm32-unknown-unknown/release
RUST_IMAGE = rust:stretch

.PHONY: all buildimage debug release doc test clean

all: release

debug: buildimage debugcgi bundleclient

release: buildimage releasecgi bundleclient

clean: buildimage cleancgi cleanclient

buildimage:
	docker build -t rust:stretch .

debugcgi:
	docker run --volume ${PWD}:/build ${RUST_IMAGE} \
		sh -c "cd /build/$(BACKEND_DIR) && cargo build" ;
	cp -u $(DEBUG_DIR)/nathanjent $(DEPLOY_DIR)/index.cgi ;
	cp -u static/.htaccess $(DEPLOY_DIR)/ ;
	cp -u static/.env $(DEPLOY_DIR)/

releasecgi:
	docker run --volume ${PWD}:/build ${RUST_IMAGE} \
		sh -c "cd /build/$(BACKEND_DIR) && cargo build --release";
	cp -u $(RELEASE_DIR)/nathanjent $(DEPLOY_DIR)/index.cgi ; 
	cp -u static/.htaccess $(DEPLOY_DIR)/ ;
	cp -u static/.env $(DEPLOY_DIR)/

bundleclient:
	cd frontend ; \
		yarn ; \
		yarn parcel build --out-dir ../$(DEPLOY_DIR) --public-url . src/index.html

doc:
	cargo doc

test:
	cargo test

cleancgi:
	docker run --volume ${PWD}:/build ${RUST_IMAGE} \
		sh -c "cd /build/$(BACKEND_DIR) && cargo clean";
	rm -f $(DEPLOY_DIR)/index.cgi

cleanclient:
	rm -rf $(DEPLOY_DIR)/*
