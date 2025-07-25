<div align="center">
  <img src=".github/assets/logo.png" alt="vrcli logo" width="200">
</div>

# vrcli

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/vrcli.svg)](https://crates.io/crates/vrcli) [![Downloads](https://img.shields.io/crates/d/vrcli.svg)](https://crates.io/crates/vrcli) [![License](https://img.shields.io/crates/l/vrcli.svg)](LICENSE) [![CI](https://github.com/VRCli/vrcli/workflows/CI/badge.svg)](https://github.com/VRCli/vrcli/actions) [![Rust Version](https://img.shields.io/badge/rustc-1.70+-blue.svg)](https://forge.rust-lang.org/infra/channel-releases.html)

</div>

A Rust CLI for VRChat: check friends, see who’s online, and send invites in one command.

## Installation

Make sure you have Rust installed (I developed this with version 1.70+, you can get the toolchain from [rustup.rs](https://rustup.rs/)).

Once that's set up, you can install `vrcli` directly from Crates.io:

```powershell
cargo install vrcli
```

## Usage

First things first, you'll need to log in to your VRChat account. This will store your credentials securely for future sessions.

```powershell
# Log in with your VRChat username and password
vrcli auth login

# You can check your login status anytime
vrcli auth status
```

Once you're logged in, most commands follow a simple `vrcli <resource> <action>` pattern.

## Commands (A Quick Overview)

Here are the main things you can do. Most commands have extra options you can see by adding `--help`.

### Authentication
- `auth login` - Connect to your VRChat account.
- `auth logout` - Clear your saved credentials.
- `auth status` - Check if you're currently logged in.

### Users
- `users search <query>` - Find users by their display name.
- `users get <identifier>` - Get public info for a user. You can use their display name, but if you want, you can use `--id` with their `usr_` ID.
  
### Friends
- `friends list` - See who's online, offline, or all your friends. Supports filtering and sorting!
- `friends get <identifier>` - Pull up the profile of a specific friend.
- `friends add <identifier>` - Send a friend request.
- `friends remove <identifier>` - Unfriend someone or cancel a request you sent.
- `friends status <identifier>` - Quickly check if a user is your friend, if you've sent them a request, etc.

### Invites
- `invite send <user> <instance_id>` - Invite a friend to a world instance.
- `invite request <user>` - Ask a friend to send you an invite to their current location.

### Worlds
- `worlds search <query>` - Look for worlds by name or author.
- `worlds get <world_id>` - Get details for a specific world using its `wrld_` ID.

### Common Options
A few useful flags work on most commands:
- `--id` - Tell the command you're providing a direct `usr_` ID to avoid a name lookup.
- `--json` - Output the raw data in JSON format. This is super handy for scripting or piping to tools like `jq`.
- `--long` / `-l` - Show a more detailed, multi-line view instead of the default compact table.

## Development

Want to hack on `vrcli`? Awesome!

```powershell
# Clone the repository
git clone https://github.com/VRCli/vrcli
cd vrcli

# This is needed for the pre-commit hooks to run
cargo install cargo-husky
```

My typical workflow looks like this:

```powershell
# Build and run tests
cargo build
cargo test

# Make sure formatting and linter checks pass before committing
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

I've set up pre-commit hooks with `cargo-husky` to automatically run `fmt` and `clippy`, so please install it to make sure your contributions pass CI.

## Contributing

Contributions are definitely welcome! If you find a bug or have an idea for a new feature, feel free to open an issue to discuss it first.

For code changes, the standard GitHub flow (fork → create a feature branch → submit a pull request) is perfect.

## License

This project is licensed under the MIT License.