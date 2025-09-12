[CLIDoc]: https://github.com/HyaenaTechnologies/configuration/blob/main/upt/documentation/upt.md
[Rust Language]: https://rust-lang.org

# System Update Tool

System Update Tool for the Advanced Package Tool, the Dandified Yellowdog Updater Modified, and Ubuntu Snap

## Features

- APT Upgrade
- DNF Upgrade
- DNF System Release Upgrade
- Snap Refresh
- Ubuntu System Releade Upgrade

## Build

- [Command Line Documentation][CLIDoc]
- [Rust][Rust Language]

```shell
git clone

cargo check

cargo build --release --target x86_64-unknown-linux-gnu

mv ./target/x86_64-unknown-linux-gnu/release/upt ./binary

./binary/upt
```

## Install System Update Daemon

```shell
sudo install ./upt /usr/local/bin/
```

