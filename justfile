# https://github.com/casey/just
# cargo install just

build:
    cargo build

test:
    @cat test/resources/diff.patch | ./target/debug/diff-rs
    
test2:
    cargo test --color=always --package diff-rs --bin diff-rs test_with_diff_file -- --nocapture --exact
