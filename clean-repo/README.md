[Rust Language]: https://rust-lang.org

# Clean Git Repositories

Clean All Git Repositories in a Directory

## Features

- Git GC

## Build

- [Rust][Rust Language]

```shell
git clone

cargo check

cargo build --release --target x86_64-unknown-linux-gnu

mv ./target/x86_64-unknown-linux-gnu/release/up-repo ./binary

./binary/clean-repo
```

## Install System Update Daemon

```shell
sudo install ./clean-repo /usr/local/bin/
```

