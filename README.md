# hcli

CLI tool for bootstrapping Zendesk Help Center projects based on the Copenhagen theme.

## Installation

### Quick Install (Linux/macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/MatheusDev20/hcli/master/install.sh | bash
```

Or with a custom install directory:

```bash
curl -fsSL https://raw.githubusercontent.com/MatheusDev20/hcli/master/install.sh | INSTALL_DIR=~/.local/bin bash
```3

### Manual Download

Download the latest binary for your platform from the [Releases](https://github.com/MatheusDev20/hc-cli/releases) page:

| Platform | Binary |
|----------|--------|
| Linux (x64) | `hcli-linux-amd64` |
| Linux (ARM64) | `hcli-linux-arm64` |
| macOS (Intel) | `hcli-macos-amd64` |
| macOS (Apple Silicon) | `hcli-macos-arm64` |
| Windows (x64) | `hcli-windows-amd64.exe` |

Then make it executable and move to your PATH:

```bash
chmod +x hcli-linux-amd64
sudo mv hcli-linux-amd64 /usr/local/bin/hcli
```

### From Source (requires Rust)

```bash
cargo install --path .
```

## Usage

```bash
hcli <PROJECT_NAME> [OUTPUT_DIR] [OPTIONS]
```

### Arguments

| Argument | Description | Default |
|----------|-------------|---------|
| `PROJECT_NAME` | Name of the project to create | Required |
| `OUTPUT_DIR` | Output directory for the project | Current directory (`.`) |

### Options

| Option | Description |
|--------|-------------|
| `--tailwind` | Include Tailwind CSS configuration |
| `-h, --help` | Print help information |
| `-V, --version` | Print version information |

## Examples

Create a new project in the current directory:

```bash
hcli my-help-center
```

Create a project in a specific directory:

```bash
hcli my-help-center /path/to/projects
```

Create a project with Tailwind CSS:

```bash
hcli my-help-center --tailwind
```

## What it does

1. Downloads the latest [Copenhagen theme](https://github.com/zendesk/copenhagen_theme) from GitHub
2. Extracts it to your project directory
3. Cleans up unnecessary files
4. Optionally sets up Tailwind CSS with PostCSS configuration
