.PHONY: shell
shell:
	nix-shell

.PHONY: test
test:
	cargo test

.PHONY: doc
doc:
	cargo doc --open

.PHONY: clean
clean:
	cargo clean
