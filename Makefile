.PHONY: deploy

target := armv7-unknown-linux-musleabihf

# tell cargo to link with an armhf compatible linker
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-ld

deploy:
	cargo build --example heading --release --target $(target)
	rsync -av target/$(target)/release/examples/heading "debian@192.168.7.2:~/"
	ssh "debian@192.168.7.2" "cd ~ && ./heading"
