[CLIDoc]: https://github.com/HyaenaTechnologies/tools-utilities/blob/main/up-repo/documentation/up-repo.md
[Rust Language]: https://rust-lang.org

# Update Repository

Update Tool for all Git Repositories in a Directory

## Features

- Git Pull in all Git Repositories in a Directory
- Git Garbage Collection in all Git Repositories in a Directory

## Build

- [Command Line Documentation][CLIDoc]
- [Rust][Rust Language]

```shell
git clone

cargo check

cargo build --release --target x86_64-unknown-linux-gnu

mv ./target/x86_64-unknown-linux-gnu/release/upt ./binary

sudo ./binary/up-repo --h
```

## Install System Update Daemon

```shell
sudo install ./up-repo /usr/local/bin/
```

