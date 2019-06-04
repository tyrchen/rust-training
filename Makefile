DIRS=$(sort $(dir $(wildcard */)))
APPS=$(filter-out target,$(DIRS:%/=%))
TESTS=$(APPS:%=%_test)

push:
	@git add .; git commit -a -n -m "${msg}"; git push;

format:
	@cargo fmt --all

lint:
	@cargo clippy --all-targets --all-features -- -D warnings

$(APPS): format
	@RUST_BACKTRACE=1 cargo run --bin $@

$(TESTS):
	@cargo test --bin $(subst _test,,$@)


.PHONY: $(APPS) $(TESTS)
