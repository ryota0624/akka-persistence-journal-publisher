COMPILE_COMMAND=cargo build --release

build:
	docker run --rm \
		-v cargo-cache:/root/.cargo \
		-v $$PWD:/volume \
		-w /volume \
		-t clux/muslrust:stable \
		sh -c "rustup target add x86_64-unknown-linux-musl && $(COMPILE_COMMAND)"

make_bootstrap:
	cp ./target/x86_64-unknown-linux-musl/release/akka-persistence-journal-publisher ./boostrap

build_test_image:
	docker build -t softprops/lambda-rust:1.51 https://github.com/softprops/lambda-rust.git#e6137ddbac36d104236407eb537c6c03a16a30fa

build_lambda_binary:
	docker run \
        -u $(id -u):$(id -g) \
        -e PACKAGE=false \
        -v ${PWD}:/code \
        -v ${HOME}/.cargo/registry:/cargo/registry \
        -v ${HOME}/.cargo/git:/cargo/git \
        --entrypoint "/bin/bash" \
        softprops/lambda-rust:1.51 \
        -c "export CARGO_HOME=/cargo && export RUSTUP_HOME=/rustup && /cargo/bin/rustup update stable && /cargo/bin/rustup default stable && /usr/local/bin/build.sh"

run_on_test_container:
	docker run \
		-i -e DOCKER_LAMBDA_STAY_OPEN=1 -p 9001:9001  \
		--rm \
		-v ${PWD}/target/lambda/release/output/akka-persistence-journal-publisher:/var/task:ro,delegated \
		lambci/lambda:provided.al2
