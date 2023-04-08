# librobotcontrol-sys [![Docs](https://docs.rs/librobotcontrol-sys/badge.svg)](https://docs.rs/librobotcontrol-sys)

A low-level library for robotics-related functionality on BeagleBone boards.

This is a [c2rust](https://github.com/immunant/c2rust) port of all of the functionality in [librobotcontrol](https://github.com/beagleboard/librobotcontrol), minus MAVLink support. For that, there's a [more idiomatic library available](https://github.com/mavlink/rust-mavlink). By using c2rust rather than raw bindings, cross-compilation is a lot cleaner.

You'll likely want to wrap whatever subset of functionality you're using in more idiomatic/safer rust.

## Example

For an example, see [`examples/heading.rs`](https://github.com/ysimonson/librobotcontrol-sys/tree/main/examples/heading.rs). This can be cross-compiled and deployed by connecting your BeagleBone and running:

```bash
# Add the target for cross-compilation
rustup target add armv7-unknown-linux-musleabihf
# Build, deploy & run on BeagleBone
make deploy
```

## c2rust

The rust code was generated via something like the following:

```bash
git clone https://github.com/beagleboard/librobotcontrol.git
cd librobotcontrol/library
cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=1
c2rust transpile --emit-build-files compile_commands.json
```

And making some manual tweaks.
