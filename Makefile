.PHONY: shell
shell:
	nix-shell

.PHONY: test
test:
	cargo test

.PHONY: doc
doc:
	cargo doc

.PHONY: clean
clean:
	cargo clean
