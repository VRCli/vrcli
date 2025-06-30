# vrcli

VRChat API CLI tool for managing friends and authentication.

## Installation

```bash
cargo install --path .
```

## Usage

### Authentication
```bash
# Login
vrcli auth login

# Check status
vrcli auth status
```

### Friends
```bash
# List friends
vrcli friends list

# Get friend details
vrcli friends get <username>
```

## Dependencies

- Rust 1.70+
- VRChat account with API access

## Development

### Quick Setup
```bash
# Windows (PowerShell)
.\scripts\dev.ps1 setup-hooks

# Linux/macOS
make dev-setup
```

### Development Commands
```bash
# Format code (always run before committing)
.\scripts\dev.ps1 format    # Windows
make format                 # Linux/macOS

# Auto-fix common clippy issues
.\scripts\dev.ps1 fix       # Windows  
make fix                    # Linux/macOS

# Run all checks
.\scripts\dev.ps1 check     # Windows
make check                  # Linux/macOS

# Build and test
.\scripts\dev.ps1 build     # Windows
make build                  # Linux/macOS
```

### Pre-commit Hook
The project includes a pre-commit hook that automatically formats code and runs linting. Install it with:
```bash
.\scripts\dev.ps1 setup-hooks    # Windows
make setup-hooks                 # Linux/macOS
```

## License

MIT