# librobotcontrol-sys [![Docs](https://docs.rs/librobotcontrol-sys/badge.svg)](https://docs.rs/librobotcontrol-sys)

A low-level library for robotics-related functionality on BeagleBone boards. You'll likely want to wrap whatever subset of functionality you're using in more idiomatic/safer rust.

See the [rust API docs for the latest release here](https://docs.rs/librobotcontrol-sys). See also the [`librobotcontrol` API docs and examples](https://beagleboard.org/static/librobotcontrol/modules.html). Things are largely the same. Where you would call, e.g.

```c
rc_button_init(...)
```

in C/C++, you'd call

```rust
unsafe { librobotcontrol_sys::rc_button_init(...) }
```

in rust.

## Example

For an example, see [`examples/heading.rs`](https://github.com/ysimonson/librobotcontrol-sys/tree/main/examples/heading.rs). This can be cross-compiled and deployed by connecting your BeagleBone and running:

```bash
# Add the target for cross-compilation
rustup target add armv7-unknown-linux-musleabihf
# Build, deploy & run on BeagleBone
make deploy-heading-example
```

## Implementation details

Figuring all of this out was a huge pain in the ass. When researching, it seemed that others had suffered through similar challenges, so maybe expounding on a subset of the details will be helpful to others. People who just want to use the damn library don't need to know all these gory details.

Unlike the default setup for `librobotcontrol`, this uses musl rather than glibc, which allows for a straight-forward cross-compilation story. But doing this is kind of tricky. Using `librobotcontrol`'s build setup would require extensive modification, and impose some very specific and difficult-to-setup toolchains on the host system. Creating a custom build setup using the `cc` crate would prevent the former, but still suffer from the latter. After several days of trying to pull off a solution with either strategy, I gave up -- if only out of concern that, even if I figured it out, it would not be at all portable.

To avoid the above issues, we build `librobotcontrol` on alpine via docker BuildKit. But newer versions of alpine use musl 1.2.x, which has 64-bit time on 32-bit machines, whereas musl 1.1.x [which is statically linked by the libc crate](https://github.com/rust-lang/libc/issues/1848) uses 32-bit time on 32-bit machines, causing linking errors. There is a [compatibility shim](https://github.com/richfelker/libcompat_time32) to address this specific issue, but it seemed easier and safer to just an older version of alpine that had musl 1.1.x. Assuming you have docker BuildKit and the appropriate VM setup (a big assumption!), regenerating the library is as simple as `make lib/librobotcontrol.a`. You don't have to do this normally because the library is included.

We also want to build the bindgen bindings ahead of time, since doing so requires the library to be installed which is unlikely to be the case on the host machine. However, the older version of alpine does not have an `apk` package for rust, and rustup doesn't offer a way to install for `armv7-unknown-linux-musleabihf` -- which is sort of surprising since we can cross-compile rust code for it fine. So to build the bindings, there's a separate alpine container using a newer version that has an `apk` package for rust.

That version of alpine also has a package for `rustfmt`, but for some reason using it in tandem with `bindgen-cli` causes the latter to never finish. So we just format the code on the host machine, after it's been copied out of the container. Rebuilding these bindings should be as simple as `make src/bindings.rs`. But again, you don't have to do this normally since the bindings are checked in.

Most of the things exported by `librobotcontrol` are not included in `bindings.rs`, because it re-exports all sorts of stuff for which there is an equivalent rust variant that's safer, e.g. libm functions. I might've been over-zealous about filtering though, so feel free to open a PR to include a missing item that you need.

An earlier version of this library used c2rust to convert `librobotcontrol` to rust, rather than trying all the shenanigans above. But I was worried about bugs from imperfect translation. And much to my horror, a bunch of stuff `#include`'d -- even libc types -- were translated _per module_. So there was e.g. a copy of the [`FILE`](https://en.cppreference.com/w/c/io/FILE) type in every module, and they weren't compatible with one another as far as rust was concerned. Earlier versions of c2rust prevented this issue [via its support for "refactorings"](https://immunant.com/blog/2019/12/header_merging/), but that was removed in more recent versions to more easily modernize it for newer versions of rust, LLVM, etc. I tried manually refactoring, but that would've given me carpal tunnel.
