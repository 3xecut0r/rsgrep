use std::env;
use std::path::{Path};
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Write, IsTerminal};
use std::io;


fn main() -> Result<()> {
    let _ = define_workflow();

    Ok(())
}

fn define_workflow() -> Result<()> {
    // Workflows:
    // 1. pattern = args[0], input = stdin
    // 2. path = args[0], pattern = args[1], input = file
    // else -> print usage and std::process::exit(2)

    // 1
    if !io::stdin().is_terminal() {
        // stdin = pipe / redirect
        let reader = BufReader::new(io::stdin().lock());
        let pattern = env::args().skip(1).next();
        if pattern.is_none() {
            panic!("There's no pattern")
        }
        let pattern = pattern.unwrap();
        let _ = grep(reader, &pattern);
        return Ok(())
    }

    // 2
    let path = env::args().skip(1).next();
    if path.is_none() {
        panic!("There's no path")
    }
    let path = path.unwrap();
    if Path::new(&path).exists() {
        let pattern = env::args().skip(2).next();
        if pattern.is_none() {
            panic!("There's no pattern")
        }
        let pattern = pattern.unwrap();

        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let _ = grep(reader, &pattern);
        return Ok(())
    }

    Ok(())
}

fn grep<R: BufRead>(mut reader: R, pattern: &str) -> Result<()> {
    for line in reader.lines() {
        let line = line?;
        if line.contains(pattern) {
            let mut out = io::stdout().lock();
            out.write_all(line.as_bytes())?;
            out.write_all(b"\n")?;
        }
    }
    Ok(())
}
