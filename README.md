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

## License

MIT