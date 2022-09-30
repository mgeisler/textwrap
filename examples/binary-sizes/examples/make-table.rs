//! Compile Textwrap with differnet featurs and record the resulting
//! binary size. Produces a Markdown table.

use std::fmt::Write;
use std::ops::Range;
use std::process::Command;
use std::{fmt, fs, io};

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

fn rustc_version() -> Result<String, PrettyError> {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .map_err(|err| PrettyError(format!("Could not determine rustc version: {err}")))?;
    let output = String::from_utf8(output.stdout)
        .map_err(|err| PrettyError(format!("Could convert output to UTF-8: {err}")))?;
    output
        .split_ascii_whitespace()
        .skip(1)
        .next()
        .map(|p| p.to_owned())
        .ok_or(PrettyError(format!(
            "Could not find rustc version in {output:?}"
        )))
}

fn make_table() -> Result<String, PrettyError> {
    let mut table = String::new();

    macro_rules! printcols {
        ($($value:expr),+) => {
            writeln!(&mut table,
                     "| {:<a$} | {:>b$} | {:>c$} |",
                     $($value),+,
                     a = 40, b = 12, c = 8)?;
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

    Ok(table)
}

struct PrettyError(String);

// Simply print the inner error with `Display` (not `Debug`) to get a
// human-readable error message.
impl fmt::Debug for PrettyError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl From<io::Error> for PrettyError {
    fn from(error: io::Error) -> Self {
        Self(error.to_string())
    }
}

impl From<fmt::Error> for PrettyError {
    fn from(error: fmt::Error) -> Self {
        Self(error.to_string())
    }
}

fn kb(size: u64) -> String {
    format!("{} KB", size / 1000)
}

fn usage() {
    println!("usage: make-table [--update PATH]");
}

fn find_marker(path: &str, marker: &str, content: &str) -> Result<Range<usize>, PrettyError> {
    let start_marker = format!("<!-- begin {marker} -->");
    let end_marker = format!("<!-- end {marker} -->");

    let start = content
        .find(&start_marker)
        .and_then(|idx| Some(idx + content[idx..].find('\n')?))
        .map(|idx| idx + b"\n".len())
        .ok_or(PrettyError(format!(
            "Could not find {start_marker:?} in {path}"
        )))?;
    let end = content
        .find(&end_marker)
        .and_then(|idx| content[..idx].rfind('\n'))
        .map(|idx| idx + b"\n".len())
        .ok_or(PrettyError(format!(
            "Could not find {end_marker:?} in {path}"
        )))?;

    Ok(start..end)
}

fn main() -> Result<(), PrettyError> {
    let args = std::env::args().collect::<Vec<String>>();
    let args = args.iter().map(|a| a.as_str()).collect::<Vec<&str>>();
    match args.as_slice() {
        &[_, "--update", path] => {
            println!("Updating {path}");
            let mut content = std::fs::read_to_string(path)?;
            let range = find_marker(&path, "binary-sizes", &content)?;
            let intro = format!(
                "With Rust {}, the size impact of the above features \
                 on your binary is as follows:\n",
                rustc_version()?
            );
            let intro = textwrap::fill(&intro, 70 - b"//! ".len());
            let table = make_table()?;
            content.replace_range(
                range,
                &textwrap::indent(&format!("\n{intro}\n{table}\n"), "//! "),
            );
            std::fs::write(path, content)?;
        }
        &[_] => println!("{}", make_table()?),
        _ => usage(),
    }
    Ok(())
}
