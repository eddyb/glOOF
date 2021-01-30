# glOOF: an OpenGL implementation experiment

## Usage

`cargo run foo ...` will run `foo ...` with a `LD_LIBRARY_PATH` containing both
`libGL.so.1` and `libGLX.so.1` symlinked to `libgloof.so`, so that glOOF can have
full control over GLX and OpenGL functions.

`cargo run glxgears` is known to work enough to be a good demonstration,
and has been the sole testcase for glOOF so far.

**Warning**: other applications that use OpenGL (especially modern ones) will likely
use functions not yet exported by `libgloof.so`, and probably crash while trying
to load them (or crash from an `unimplemented!(...)` panic later, instead).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
