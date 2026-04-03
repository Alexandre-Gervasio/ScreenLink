# KVM Pro - Contributor Guide

## Setting Up Development Environment

### Prerequisites

- **Rust 1.70+**: https://rustup.rs/
- **Git**
- **Platform-specific tools**:
  - **Linux**: `libevdev-dev` (Ubuntu/Debian) or `libevdev-devel` (Fedora)
  - **Windows**: Visual Studio Build Tools or MinGW

### Quick Setup

```bash
# Clone repository
git clone https://github.com/yourusername/kvm-pro.git
cd kvm-pro

# Run setup script (Linux/macOS)
chmod +x scripts/setup.sh
./scripts/setup.sh

# Windows - Install dependencies manually
# Then:
cd core
cargo build --release
```

## Project Structure

```
core/
├── src/
│   ├── lib.rs              # Public library API
│   ├── main.rs             # Server binary
│   ├── client.rs           # Client binary
│   ├── input/              # Event capture/injection
│   ├── network/            # TCP/UDP communication
│   ├── security/           # TLS/encryption
│   ├── config/             # Configuration management
│   ├── plugins/            # Plugin system
│   ├── screen/             # Multi-monitor support
│   ├── clipboard/          # Clipboard sync
│   └── utils/              # Utilities
├── tests/                  # Integration tests
└── Cargo.toml

shared/
├── protocol.rs             # Protocol definitions
└── constants.rs            # Global constants

scripts/
├── build.sh                # Build script
├── package.sh              # Packaging script
└── setup.sh                # Development setup
```

## Architecture

### Event Flow

```
CAPTURE (Linux: evdev, Windows: Raw Input)
    ↓
SERIALIZE (bincode)
    ↓
SEND (TCP)
    ↓
DESERIALIZE
    ↓
INJECT (Linux: uinput, Windows: SendInput)
```

### Module Responsibilities

- **input**: Platform-specific event capture and injection
- **network**: TCP server/client and UDP discovery
- **security**: TLS configuration and certificate management
- **config**: TOML file parsing and configuration
- **plugins**: Plugin loading and event processing
- **screen**: Monitor layout and boundary detection
- **clipboard**: System clipboard access
- **utils**: Serialization and utility functions

## Code Style

- Follow Rust conventions (rustfmt)
- Use meaningful error types (prefer `anyhow` or custom `Error`)
- Log important events (use `log` crate)
- Write tests for new features
- Document public APIs with doc comments

## Common Tasks

### Running Tests

```bash
cd core
cargo test
cargo test -- --nocapture  # With output
```

### Running with Debug Output

```bash
cd core
RUST_LOG=debug cargo run --bin kvm-pro-server
RUST_LOG=debug cargo run --bin kvm-pro-client
```

### Checking Code Quality

```bash
cd core
cargo fmt              # Format code
cargo clippy           # Lint checks
```

### Building for Release

```bash
./scripts/build.sh
```

## Adding a New Feature

1. Create a new file in the appropriate module
2. Add public interface in `mod.rs`
3. Write tests
4. Update documentation
5. Submit a PR with clear description

Example: Adding keyboard layout support

```rust
// core/src/input/keymap.rs
pub struct KeymapManager {
    current_layout: String,
    layouts: HashMap<String, Keymap>,
}

impl KeymapManager {
    pub fn new() -> Self {
        Self {
            current_layout: "us".to_string(),
            layouts: HashMap::new(),
        }
    }

    pub fn translate_keycode(&self, keycode: u32) -> u32 {
        // Map keycode based on current layout
        keycode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keymap() {
        let manager = KeymapManager::new();
        assert_eq!(manager.current_layout, "us");
    }
}
```

Then update `core/src/input/mod.rs`:

```rust
pub mod keymap;
pub use keymap::KeymapManager;
```

## Debugging

### Using rust-gdb (Linux)

```bash
cd core
rust-gdb ./target/debug/kvm-pro-server
```

### Print Debugging

```rust
log::debug!("Variable value: {:?}", my_var);
```

### Finding Memory Issues

```bash
cd core
cargo build
RUST_BACKTRACE=1 cargo run
```

## Performance Profiling

### Linux

```bash
# Install perf
sudo apt-get install linux-tools-generic

# Profile
perf record -g ./target/release/kvm-pro-server
perf report
```

### All Platforms

```bash
cargo build --release
# Binary is optimized and ready for testing
```

## Creating a Plugin

Plugins implement the `Plugin` trait:

```rust
use kvm_pro::plugins::Plugin;
use kvm_pro::InputEvent;

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "My Plugin"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn on_event(&mut self, event: InputEvent) -> InputEvent {
        // Process event
        event
    }

    fn on_load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize
        Ok(())
    }

    fn on_unload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Cleanup
        Ok(())
    }
}
```

## Submitting a Pull Request

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Test: `cargo test`
5. Format: `cargo fmt`
6. Commit: `git commit -m "Add my feature"`
7. Push: `git push origin feature/my-feature`
8. Open a PR with:
   - Clear title
   - Description of changes
   - Why it's needed
   - Any breaking changes

## Roadmap

See [README.md](../README.md#roadmap) for project roadmap.

## Getting Help

- **Documentation**: See README.md
- **Issues**: Create a GitHub issue
- **Discussions**: Use GitHub Discussions
- **Code**: Check examples in `plugins/`

## Contact

For questions, open a GitHub Discussion or issue.

---

Happy coding!
