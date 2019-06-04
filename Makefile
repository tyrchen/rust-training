DIRS=$(sort $(dir $(wildcard */)))
RUNS=$(DIRS:%/=%_run)
TESTS=$(DIRS:%/=%_test)

push:
	@git add .; git commit -a -n -m "${msg}"; git push;

format:
	@cargo fmt --all

lint:
	@cargo clippy --all-targets --all-features -- -D warnings

$(RUNS): format
	@RUST_BACKTRACE=1 cargo run --bin $(subst _run,,$@)

$(TESTS):
	@cargo test --bin $(subst _test,,$@)


.PHONY: $(RUNS) $(TESTS)
