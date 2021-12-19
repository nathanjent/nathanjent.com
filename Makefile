BACKEND_DIR = backend
DEPLOY_DIR = www
WASM_DIR =  target/wasm32-unknown-unknown/release
RUST_BUILDER_IMG = rust:stretch

.PHONY: all buildimage debug release doc test clean deploydir config

all: release

debug: buildimage debugcgi bundleclient

release: buildimage releasecgi bundleclient

clean: buildimage cleancgi cleanclient

buildimage:
	docker build -t $(RUST_BUILDER_IMG) .

$(DEPLOY_DIR):
	mkdir -p $(DEPLOY_DIR)

config:
	cp -u static/.htaccess $(DEPLOY_DIR)/ ;
	cp -u static/.env $(DEPLOY_DIR)/

debugcgi: $(DEPLOY_DIR) config
	docker run --volume ${PWD}:/build $(RUST_BUILDER_IMG) \
		sh -c "cd /build/$(BACKEND_DIR) && cargo build" ;
	cp -u ${BACKEND_DIR}/target/debug/nathanjent $(DEPLOY_DIR)/index.cgi ;

releasecgi: $(DEPLOY_DIR) config
	docker run --volume ${PWD}:/build $(RUST_BUILDER_IMG) \
		sh -c "cd /build/$(BACKEND_DIR) && cargo build --release";
	cp -u $(BACKEND_DIR)/target/release/nathanjent $(DEPLOY_DIR)/index.cgi ; 

bundleclient:
	cd frontend ; \
		yarn ; \
		yarn parcel build --out-dir ../$(DEPLOY_DIR) --public-url . src/index.html

doc:
	cargo doc

test:
	cargo test

cleancgi:
	docker run --volume ${PWD}:/build ${RUST_BUILDER_IMG} \
		sh -c "cd /build/$(BACKEND_DIR) && cargo clean";
	rm -f $(DEPLOY_DIR)/index.cgi

cleanclient:
	rm -rf $(DEPLOY_DIR)/*
