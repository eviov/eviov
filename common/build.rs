use std::{env, fs, io, path::PathBuf};

fn main() -> io::Result<()> {
    use io::{BufRead, Write};

    let input = fs::File::open(
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../walkthrough.md"),
    )
    .unwrap();
    let input = io::BufReader::new(input);
    let mut output =
        fs::File::create(PathBuf::from(env::var("OUT_DIR").unwrap()).join("walkthrough.rs"))
            .unwrap();
    for line in input.lines() {
        writeln!(&mut output, "/// {}", line?)?;
    }
    writeln!(output, "pub mod walkthrough{{}}")
}
