CARGO=cargo
CARGO_NIGHTLY=rustup run nightly cargo

build: gen
	${CARGO} build --color=always --all --all-targets

build-nightly: gen
	${CARGO_NIGHTLY} build --color=always --all --all-targets

gen:
	protocol-generator/bin/gen_protocol_code.sh

test:
	RUST_BACKTRACE=1 ${CARGO} test -- --nocapture

test-nightly:
	RUST_BACKTRACE=1 ${CARGO_NIGHTLY} test -- --nocapture

clean:
	protocol-generator/bin/clean_protocol_code.sh
	${CARGO} clean --target-dir=protocol-generator/target
	${CARGO} clean
