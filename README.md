<div align="center">
  <img src=".github/assets/logo.png" alt="vrcli logo" width="200">
</div>

# vrcli

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/vrcli.svg)](https://crates.io/crates/vrcli) [![Downloads](https://img.shields.io/crates/d/vrcli.svg)](https://crates.io/crates/vrcli) [![License](https://img.shields.io/crates/l/vrcli.svg)](LICENSE) [![CI](https://github.com/VRCli/vrcli/workflows/CI/badge.svg)](https://github.com/VRCli/vrcli/actions) [![Rust Version](https://img.shields.io/badge/rustc-1.70+-blue.svg)](https://forge.rust-lang.org/infra/channel-releases.html)

</div>

A command-line interface for the VRChat API that lets you manage friends, users, worlds, and authentication directly from your terminal.

## Installation

First, install Rust (https://www.rust-lang.org/tools/install), then:

```powershell
cargo install vrcli
```

## Usage

Set up authentication:

```powershell
vrcli auth login
vrcli auth status
```

General syntax:

```
vrcli <resource> <action> [options]
```

## Commands

### Authentication
- `auth login`
- `auth logout`
- `auth status`

### Users
- `users list`
- `users get <username>`
- `users diagnose <username>`

### Friends
- `friends list`
- `friends add <username>`
- `friends remove <username>`

### Worlds
- `worlds search <query> [--featured]`
- `worlds get <world_id> [--json]`

### Configuration
- `config view`
- `config set <key> <value>`

## Development

Clone the repo and install dependencies:

```powershell
git clone https://github.com/your/repo.git
cd vrcli
cargo install cargo-husky
```

Standard workflow:

```powershell
# Build and test
cargo build
cargo test

# Format and lint
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

Pre-commit hooks run automatically via cargo-husky.

## Contributing

Contributions are welcome! Please fork the repo, create a branch, and submit a pull request.

## License

MIT
