use std::{
    fs,
    path::{Path, PathBuf},
};

use xshell::{cmd, Shell};

pub fn sync(file: &Path, contents: &str) {
    if let Ok(old_contents) = fs::read_to_string(file) {
        if old_contents == contents {
            return;
        }
    }

    println!("Updating {}", file.to_str().unwrap());

    fs::write(file, contents).unwrap();
}

pub fn reformat(text: String) -> String {
    let sh = Shell::new().unwrap();

    ensure_rustfmt(&sh);

    let mut stdout = cmd!(sh, "rustup run stable rustfmt")
        .stdin(text)
        .read()
        .unwrap();

    if !stdout.ends_with('\n') {
        stdout.push('\n');
    }

    stdout
}

pub fn workspace_dir() -> PathBuf {
    let current_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_dir = PathBuf::from(current_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned();

    // Just to confirm we are in the right dir.
    assert!(workspace_dir.join("LICENSE").exists());

    workspace_dir
}

pub fn to_upper_snake_case(s: &str) -> String {
    let mut buf = String::with_capacity(s.len());
    let mut prev = false;
    for c in s.chars() {
        if c.is_ascii_uppercase() && prev {
            buf.push('_')
        }
        prev = true;

        buf.push(c.to_ascii_uppercase());
    }
    buf
}

fn ensure_rustfmt(sh: &Shell) {
    let version = cmd!(sh, "rustup run stable rustfmt --version")
        .read()
        .unwrap_or_default();
    if !version.contains("stable") {
        panic!(
            "Failed to run rustfmt from toolchain 'stable'. \
                 Please run `rustup component add rustfmt --toolchain stable` to install it.",
        );
    }
}
