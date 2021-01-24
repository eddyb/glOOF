# glOOF: an OpenGL implementation experiment

## Usage

`cargo run foo ...` will run `foo ...` with `LD_PRELOAD` set to `libgloof.so`,
so that GLX and OpenGL functions can be overriden.

`cargo run glxgears` is known to work enough to be a good demonstration,
and has been the sole testcase for glOOF so far.

**Warning**: other applications that use OpenGL (especially modern ones) will likely use functions not yet overriden by `libgloof.so`, and probably crash.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
