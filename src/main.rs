use std::env;
use std::path::PathBuf;
use std::process::{self, Command};

// https://github.com/rust-lang/cargo/blob/1857880b5124580c4aeb4e8bc5f1198f491d61b1/src/cargo/util/paths.rs#L29-L52
fn dylib_path_envvar() -> &'static str {
    if cfg!(windows) {
        "PATH"
    } else if cfg!(target_os = "macos") {
        "DYLD_FALLBACK_LIBRARY_PATH"
    } else {
        "LD_LIBRARY_PATH"
    }
}

fn find_gloof_dylib() -> PathBuf {
    let filename = format!(
        "{}gloof{}",
        env::consts::DLL_PREFIX,
        env::consts::DLL_SUFFIX
    );
    let var_name = dylib_path_envvar();
    if let Some(var) = env::var_os(var_name) {
        for mut path in env::split_paths(&var) {
            path.push(&filename);
            if path.is_file() {
                return path;
            }
        }
    }
    unreachable!("{} can't be found in {}", filename, var_name);
}

fn main() {
    let mut args = env::args_os();
    let exe = args.next().unwrap();
    let program = args
        .next()
        .expect(&format!("Usage: {} <program>", exe.to_string_lossy()));

    // FIXME(eddyb) try to get this working cross-platform.
    let gloof = find_gloof_dylib();
    for lib_name in &["libGL.so.1", "libGLX.so.1"] {
        let lib = gloof.with_file_name(lib_name);
        let _ = std::fs::remove_file(&lib);
        std::os::unix::fs::symlink(&gloof, lib).unwrap();
    }
    let status = Command::new(program).args(args).status().unwrap();
    if let Some(code) = status.code() {
        process::exit(code);
    }
}
