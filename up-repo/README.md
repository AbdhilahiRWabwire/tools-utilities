[Rust Language]: https://rust-lang.org

# Update Git Repositories

Update All Git Repositories in a Directory

## Features

- Git Pull

## Build

- [Rust][Rust Language]

```shell
git clone

cargo check

cargo build --release --target x86_64-unknown-linux-gnu

mv ./target/x86_64-unknown-linux-gnu/release/up-repo ./binary

sudo ./binary/up-repo au
```

## Install System Update Daemon

```shell
sudo install ./up-repo /usr/local/bin/
```

