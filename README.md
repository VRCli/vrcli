<div align="center">
  <img src=".github/assets/logo.png" alt="vrcli logo" width="200">
</div>

# vrcli

A command-line interface for the VRChat API that lets you manage friends, users, worlds, and authentication directly from your terminal.

## Features

- **Friend Management**: List, search, add, and remove friends with extensive filtering and display options
- **User Operations**: Search users, manage notes, view feedback, and diagnose access issues
- **World Discovery**: Search and explore VRChat worlds with detailed information
- **Secure Authentication**: OAuth2-based login with 2FA support and session management
- **Flexible Output**: Both human-readable tables and JSON output for automation
- **Cross-Platform**: Works on Windows, macOS, and Linux

## Installation

### From Cargo
```bash
cargo install vrcli
```

### From Source
```bash
git clone https://github.com/VRCli/vrcli
cd vrcli
cargo install --path .
```

### Prerequisites
- Rust 1.70 or later
- Valid VRChat account

## Quick Start

1. **Authenticate** with your VRChat account:
   ```bash
   vrcli auth login
   ```

2. **List your friends**:
   ```bash
   vrcli friends list
   ```

3. **Search for users**:
   ```bash
   vrcli users search "username"
   ```

4. **Explore worlds**:
   ```bash
   vrcli worlds search "world name"
   ```

## Command Reference

### Authentication (`vrcli auth`)

| Command  | Description                                      |
| -------- | ------------------------------------------------ |
| `login`  | Interactive login with password or authcookie_   |
| `status` | Show current authentication status and user info |

### Friends Management (`vrcli friends`)

| Command               | Description                                 | Example                                     |
| --------------------- | ------------------------------------------- | ------------------------------------------- |
| `list`                | List all friends with filtering and sorting | `vrcli friends list --online --sort status` |
| `get <identifier>`    | Get detailed friend information             | `vrcli friends get "username"`              |
| `add <identifier>`    | Send a friend request                       | `vrcli friends add "username"`              |
| `remove <identifier>` | Remove friend or cancel request             | `vrcli friends remove "username"`           |
| `status <identifier>` | Check friendship status                     | `vrcli friends status "username"`           |

#### Friends List Options
- `--online` / `--offline`: Filter by online status
- `--limit <N>`: Limit number of results
- `--sort <method>`: Sort by name, status, activity, platform, or id
- `--reverse`: Reverse sort order
- `--show-id`, `--show-status`, `--show-platform`, `--show-location`, `--show-activity`: Display additional columns
- `--json`: Output as JSON

### User Operations (`vrcli users`)

| Command                        | Description                        | Example                                     |
| ------------------------------ | ---------------------------------- | ------------------------------------------- |
| `search <query>`               | Search users by display name       | `vrcli users search "partial name"`         |
| `get <identifier>`             | Get user information by ID or name | `vrcli users get "username"`                |
| `get-by-name <username>`       | Get user by exact username         | `vrcli users get-by-name "exact_username"`  |
| `note get <identifier>`        | Get your note for a user           | `vrcli users note get "username"`           |
| `note set <identifier> <note>` | Set/update note for a user         | `vrcli users note set "username" "My note"` |
| `notes`                        | List all your user notes           | `vrcli users notes`                         |
| `feedback <identifier>`        | Get feedback/moderation info       | `vrcli users feedback "username"`           |
| `diagnose <identifier>`        | Troubleshoot user access issues    | `vrcli users diagnose "username"`           |

### World Discovery (`vrcli worlds`)

| Command          | Description                    | Example                                                        |
| ---------------- | ------------------------------ | -------------------------------------------------------------- |
| `search <query>` | Search worlds by name          | `vrcli worlds search "avatar world"`                           |
| `get <world_id>` | Get detailed world information | `vrcli worlds get "wrld_12345678-1234-1234-1234-123456789012"` |

#### World Search Options
- `--featured`: Show only featured worlds
- `--limit <N>`: Number of results (default: 20)
- `--offset <N>`: Pagination offset
- `--long`: Show detailed information
- `--json`: Output as JSON

## Output Formats

### Table Output (Default)
Human-readable tables with customizable columns:
```
Name                 Status     Platform Location    
ExampleUser          Online     PC       Private     
AnotherFriend        Offline    Quest    N/A         
```

### JSON Output
Machine-readable JSON for scripting and automation:
```bash
vrcli friends list --json | jq '.[] | select(.status == "Online")'
```

## Configuration

Authentication credentials are securely stored in:
- **Windows**: `%APPDATA%\vrcli\`
- **macOS**: `~/Library/Application Support/vrcli/`
- **Linux**: `~/.config/vrcli/`

## Common Use Cases

### Monitor Online Friends
```bash
# Show online friends sorted by status
vrcli friends list --online --sort status --show-status

# Get detailed info about a specific friend
vrcli friends get "username" --json
```

### User Research
```bash
# Search for users and get detailed info
vrcli users search "artist" --long
vrcli users get "specific_user" --json

# Manage your notes
vrcli users note set "username" "Great artist, collaborated on project X"
vrcli users notes --long
```

### World Discovery
```bash
# Find featured worlds
vrcli worlds search "party" --featured --long

# Get detailed world information
vrcli worlds get "wrld_12345678-1234-1234-1234-123456789012" --json
```

### Troubleshooting
```bash
# Diagnose why you can't access a user's profile
vrcli users diagnose "username"

# Check authentication status
vrcli auth status
```

## Development

### Setting Up the Development Environment

1. **Clone the repository**:
   ```bash
   git clone https://github.com/VRCli/vrcli
   cd vrcli
   ```

2. **Install dependencies**:
   ```bash
   cargo build
   ```

3. **Set up pre-commit hooks** (optional but recommended):
   ```bash
   cargo install cargo-husky
   ```

### Local Development Commands

To ensure your code passes CI checks before pushing, use these commands:

**Run the exact same checks as CI** (recommended before each commit):
```bash
# Cross-platform (Make)
make ci-local

# Windows PowerShell
.\scripts\dev.ps1 ci-local

# Manual commands
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --verbose
```

**Individual checks**:
```bash
# Format code
cargo fmt --all

# Run clippy (same as CI)
cargo clippy --all-targets --all-features -- -D warnings

# Run clippy with auto-fix
cargo clippy --fix --all-targets --all-features -- -D warnings

# Run tests
cargo test --verbose
```

**Auto-fix common issues**:
```bash
make fix  # or .\scripts\dev.ps1 fix
```

### VS Code Setup

The repository includes VS Code settings that:
- Enable Clippy on save with the same strictness as CI
- Auto-format code on save
- Show Clippy warnings inline as you type

Make sure you have the `rust-analyzer` extension installed.

### Pre-commit Hooks

The project uses `cargo-husky` to run the same Clippy checks locally before each commit. This prevents CI failures due to linting issues.

If you want to bypass the pre-commit hook temporarily:
```bash
git commit --no-verify -m "your message"
```

### CI Consistency

The project is configured to ensure local and CI environments behave identically:
- `clippy.toml` enforces consistent linting rules
- VS Code settings match CI Clippy configuration
- `make ci-local` runs the exact same commands as GitHub Actions

### Development Workflow

#### Formatting and Linting
```bash
# Windows (PowerShell)
.\scripts\dev.ps1 format     # Format code
.\scripts\dev.ps1 clippy     # Run linter
.\scripts\dev.ps1 fix        # Auto-fix common issues

# Linux/macOS
make format                  # Format code
make clippy                  # Run linter
make fix                     # Auto-fix common issues
```

#### Building and Testing
```bash
# Windows (PowerShell)
.\scripts\dev.ps1 build      # Debug build
.\scripts\dev.ps1 test       # Run tests
.\scripts\dev.ps1 check      # Full check pipeline

# Linux/macOS
make build                   # Debug build
make test                    # Run tests
make check                   # Full check pipeline
```

#### CI/CD Simulation
```bash
# Run the exact same checks as GitHub Actions
.\scripts\dev.ps1 ci-local   # Windows
make ci-local                # Linux/macOS
```

### Git Hooks
The project includes automatic pre-commit hooks that:
- Format code with `cargo fmt`
- Run linter with `cargo clippy`
- Ensure code quality before commits

Install hooks with:
```bash
.\scripts\dev.ps1 setup-hooks    # Windows
make setup-hooks                 # Linux/macOS
```

### Project Structure
```
src/
├── main.rs              # CLI argument parsing and main entry point
├── commands.rs          # Command module exports
├── config.rs            # Configuration management
├── commands/
│   ├── auth/            # Authentication commands
│   ├── friends/         # Friend management commands
│   ├── users/           # User operation commands
│   └── worlds/          # World discovery commands
└── common/              # Shared utilities and helpers
    ├── auth_client.rs   # Authenticated API client
    ├── formatter.rs     # Output formatting
    ├── table.rs         # Table display utilities
    └── ...
```

### Dependencies

#### Core Dependencies
- `vrchatapi` (1.20.0) - VRChat API client library
- `clap` (4.0) - Command-line argument parsing
- `tokio` (1.0) - Async runtime
- `anyhow` (1.0) - Error handling
- `serde` (1.0) - Serialization/deserialization

#### UI and Display
- `colored` (2.0) - Terminal color output
- `unicode-width` (0.1) - Unicode text width calculation
- `inquire` (0.7) - Interactive prompts

#### Development Tools
- `cargo-husky` (1) - Git hooks integration

### Contributing

1. **Fork and clone** the repository
2. **Install development tools**: `.\scripts\dev.ps1 setup-hooks` (Windows) or `make dev-setup` (Linux/macOS)
3. **Make your changes** with proper formatting and testing
4. **Run full checks**: `.\scripts\dev.ps1 check` or `make check`
5. **Submit a pull request**

All contributions must pass the CI pipeline which includes:
- Code formatting (`cargo fmt`)
- Linting (`cargo clippy`)
- All tests passing (`cargo test`)

## Security

- Authentication tokens are stored securely in OS-appropriate locations
- Support for 2FA authentication
- No plaintext password storage
- Session management with automatic token refresh

## Troubleshooting

### Common Issues

**Authentication failures:**
```bash
vrcli auth status  # Check current authentication
vrcli auth login   # Re-authenticate
```

**User not found errors:**
```bash
vrcli users diagnose "username"  # Diagnose access issues
```

**API rate limiting:**
- The tool respects VRChat API rate limits
- Use `--limit` options to control request volume
- Consider pagination with `--offset` for large datasets

### Debug Information

For detailed error information, set the `RUST_LOG` environment variable:
```bash
# Windows
$env:RUST_LOG="debug"; vrcli friends list

# Linux/macOS
RUST_LOG=debug vrcli friends list
```

## License

MIT