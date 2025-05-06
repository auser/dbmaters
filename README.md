# DBmaters

DBmaters is a Rust library that provides a C API for database migrations using the fantastic [dbmate](https://github.com/amacneil/dbmate) project.

## Prerequisites

This library requires the Go library to be built first. Run:

```bash
make build-go-static
```

## Building

```bash
cargo build
```

## Usage

```rust
use dbmaters;

fn main() {
    dbmaters::create_and_migrate("postgres://localhost/mydb");
}
```

