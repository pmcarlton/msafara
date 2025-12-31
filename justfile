format:
    rustfmt src/**/*.rs

tags:
    ctags -R --exclude='data/*' --exclude='target/*'

test:
	cargo test --color=always --no-fail-fast
