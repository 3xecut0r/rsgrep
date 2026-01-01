# rsgrep

A tine `grep`-like tool written in Rust for learning purposes.

It searching for a fixed substring in input and prints matching lines to stdout.
Input can come from file or from `stdin` (pipes / redirects).

## Features

- Fixed substring search (`line.contains(pattern)`)
- Read from :
  - `stdin` via pipe/redirect
  - a single file path
- Prints matching lines to `stdout`

## Build 


```bash
cargo build --release
```
Binary would be at:
```
./target/release/rsgrep
```

## Usage

### 1) Search in a file

```bash
./target/release/rsgrep <file> <pattern>
```

### 2) Search in stdin (pipe / redirect)

```bash
cat file.txt | ./target/release/rsgrep <pattern>
```

Or redirect:

```bash
./target/release/rsgrep error < file.txt
```
