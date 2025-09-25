# MicVolumeHolder

A Windows utility that maintains microphone volume at a specified level. It automatically adjusts the volume of all active microphones to prevent unwanted volume changes.

## Features

- 🎤 Maintains consistent microphone volume levels
- ⚡ Light-weight and efficient
- 🔄 Configurable adjustment intervals
- 📊 Supports multiple active microphones
- 🪟 Native Windows audio API integration

## Installation

### From Releases

1. Download the latest `MicVolumeHolder.exe` from [GitHub Releases](../../releases)
2. Run the executable directly - no installation needed

### Building from Source

1. Ensure you have [Rust installed](https://rustup.rs/)
2. Clone the repository:

```bash
git clone https://github.com/yourusername/MicVolumeHolder
cd MicVolumeHolder
```

3. Build with cargo:

```bash
cargo build --release
```

The executable will be available at `target/release/MicVolumeHolder.exe`

## Usage

```bash
MicVolumeHolder.exe [OPTIONS]
```

### Options

- `-v, --volume <VOLUME>` - Target volume percentage (0-100)
  - Default: 100%
- `-t, --time <TIME>` - Adjustment interval in seconds
  - Default: 5.0 seconds

### Examples

Keep microphone at 80% volume, checking every 10 seconds:

```bash
MicVolumeHolder.exe --volume 80 --time 10
```

Use default settings (100% volume, 5-second intervals):

```bash
MicVolumeHolder.exe
```

## Development

### Prerequisites

- Rust toolchain (stable)
- Windows development environment

### Build and Test

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run with cargo
cargo run -- --volume 90 --time 3
```

### Release Process

1. Update version in `Cargo.toml`
2. Create and push a new tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

3. GitHub Actions will automatically:
   - Build the release executable
   - Create a GitHub Release
   - Upload the binary as a release asset

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [windows-rs](https://github.com/microsoft/windows-rs) for Windows API bindings
- Command-line interface powered by [clap](https://github.com/clap-rs/clap)
