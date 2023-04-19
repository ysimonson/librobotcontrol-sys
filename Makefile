.PHONY: init deploy-heading-example

target := armv7-unknown-linux-musleabihf

# tell cargo to link with an armhf compatible linker
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-ld

init:
	git submodule update --init
	git lfs install
	git lfs pull

lib/librobotcontrol.a:
	docker buildx build -f lib.Dockerfile --progress=plain --output=lib .

src/bindings.rs:
	docker buildx build -f src.Dockerfile --progress=plain --output=src .
	cargo fmt

deploy-heading-example:
	cargo build --example heading --release --target $(target)
	rsync -av target/$(target)/release/examples/heading "debian@192.168.7.2:~/"
	ssh "debian@192.168.7.2" "cd ~ && ./heading"
