// HACK(eddyb) this is used in a couple places, and can't easily be just a
// `const` because it's used as e.g. `concat!(version_str!(major.minor), "\0")`.
macro_rules! version_str {
    (major.minor) => {
        concat!(
            env!("CARGO_PKG_VERSION_MAJOR"),
            ".",
            env!("CARGO_PKG_VERSION_MINOR")
        )
    };
}

mod gl;
mod glx;
