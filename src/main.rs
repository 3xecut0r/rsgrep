use std::env;
use std::path::{Path};
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Write, IsTerminal};
use std::io;
use std::process;


fn main() -> Result<()> {
    let _ = define_workflow();

    Ok(())
}

fn define_workflow() -> Result<()> {
    // Workflows:
    // 1. pattern = args[0], input = stdin
    // 2. path = args[0], pattern = args[1], input = file
    // else -> print usage and std::process::exit(2)
    let args: Vec<String> = env::args().skip(1).collect();

    // 1
    if !io::stdin().is_terminal() {
        // stdin = pipe / redirect
        let stdin = io::stdin().lock();
        let reader = BufReader::new(stdin);

        if args.len() < 1 {
            usage("There's no pattern", 2);
        }
        let pattern = &args[0];
        let _ = grep(reader, pattern);
        return Ok(())
    }

    // 2
    if args.len() < 1 {
        usage("There's no path", 2);
    }
    let path = &args[0];
    if Path::new(&path).exists() {

        if args.len() < 2 {
            usage("There's no pattern", 2);
        }
        let pattern = &args[1];
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let _ = grep(reader, pattern);
        return Ok(())
    } else {
        usage("Can not open / Does not exist a file. / Reading Error", 2);
    }

    Ok(())
}

fn grep<R: BufRead>(mut reader: R, pattern: &str) -> Result<()> {
    let mut out = io::stdout().lock();
    let mut line = String::new();

    loop {
        line.clear();
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            break; // EOF
        }

        if line.contains(pattern) {
            out.write_all(line.as_bytes())?;
        }
    }

    Ok(())
}

fn usage(message: &str, code: i32) {
    eprintln!("{}", message);
    process::exit(code);
}

