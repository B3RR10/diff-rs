COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO ?= cargo --color $(COLOR)
WATCH ?= cargo watch -c -x

.PHONY: all bench build check clean doc install publish run test update

all: build

bench:
	@$(CARGO) bench

build:
	@$(CARGO) build

check:
	@$(CARGO) check

clean:
	@$(CARGO) clean

doc:
	@$(CARGO) rustdoc --bin diff-rs --open -- --document-private-items
	@xdg-open target/doc/diff_rs/index.html

install: build
	@$(CARGO) install

publish:
	@$(CARGO) publish

run: build
	@$(CARGO) run

watch:
	@$(WATCH) check

test: test1 test2 test3

update:
	@$(CARGO) update

test1: build
	@cat test/resources/diff.patch | cargo run
    
test2: build
	@$(CARGO) test --package diff-rs --bin diff-rs test_with_diff_file -- --nocapture --exact

test3: build
	@$(CARGO) test --package diff-rs --bin diff-rs test_print_file -- --nocapture --exact
