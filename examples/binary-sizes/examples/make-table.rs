//! Compile Textwrap with differnet featurs and record the resulting
//! binary size. Produces a Markdown table.

use std::process::Command;
use std::{fs, io};

fn compile(extra_args: &[&str]) -> io::Result<u64> {
    let status = Command::new("cargo")
        .arg("build")
        .arg("--quiet")
        .arg("--release")
        .args(extra_args)
        .status()?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("failed to compile: {}", status),
        ));
    }

    let path = "target/release/textwrap-binary-sizes-demo";
    let status = Command::new("strip").arg(path).status()?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("failed to strip: {}", status),
        ));
    }

    let metadata = fs::metadata(path).map_err(|err| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("failed to read metadata for {}: {}", path, err),
        )
    })?;
    Ok(metadata.len())
}

struct PrettyError {
    error: io::Error,
}

// Simply print the inner error with `Display` (not `Debug`) to get a
// human-readable error message.
impl std::fmt::Debug for PrettyError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.error)
    }
}

impl From<io::Error> for PrettyError {
    fn from(error: io::Error) -> Self {
        Self { error }
    }
}

fn kb(size: u64) -> String {
    format!("{} KB", size / 1000)
}

fn main() -> Result<(), PrettyError> {
    macro_rules! printcols {
        ($($value:expr),+) => {
            println!("| {:<a$} | {:>b$} | {:>c$} |",
                     $($value),+,
                     a = 40, b = 12, c = 8);
        };
    }

    printcols!("Configuration", "Binary Size", "Delta");
    printcols!(":---", "---:", "---:");

    let features = [
        ("textwrap", "textwrap without default features"),
        ("smawk", "textwrap with smawk"),
        ("unicode-width", "textwrap with unicode-width"),
        ("unicode-linebreak", "textwrap with unicode-linebreak"),
    ];
    let base_size = compile(&[])?;
    printcols!("quick-and-dirty implementation", kb(base_size), "— KB");

    for (feature, label) in features.iter() {
        let size = compile(&["--features", feature])?;
        let delta = size - base_size;
        printcols!(label, kb(size), kb(delta));
    }

    Ok(())
}
