.PHONY: init deploy-heading-example

target := armv7-unknown-linux-musleabihf

# tell cargo to link with an armhf compatible linker
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-ld

init:
	git submodule update --init

bindings:
	docker buildx build  --progress=plain --output=bindings .
	mv bindings/bindings.rs src/bindings.rs && cargo fmt
	mkdir -p lib
	mv bindings/librobotcontrol.a lib/

deploy-heading-example:
	cargo build --example heading --release --target $(target)
	rsync -av target/$(target)/release/examples/heading "debian@192.168.7.2:~/"
	ssh "debian@192.168.7.2" "cd ~ && ./heading"
