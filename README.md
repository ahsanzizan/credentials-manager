# Credentials Manager

Simple credentials manager built with Rust

# Usage

Build the project

```bash
cargo build --release
```

### Add new credential:

```bash
./target/release/credential-manager add <username> <password>
```

### Get existing credential:

```bash
./target/release/credential-manager get <username>
```
