# Project Aurora

Turn the Kobo Clara BW into the ultimate hackable Linux-powered eReader **without replacing Nickel**.

## Overview

Project Aurora is a complete developer ecosystem for Kobo that includes:

- **Browser Dashboard** — web-based device management (terminal, file browser, system monitor)
- **Package Manager** — install, remove, and update apps/packages
- **App Store** — discover and install community apps
- **Plugin SDK** — build extensions in Lua, Shell, Qt, or REST
- **Widget Framework** — FbInk-based on-screen widgets
- **Background Services** — SSH, Git sync, OTA updates, notifications
- **Native Apps** — Qt applications for productivity and reading

## Getting Started

### Prerequisites

- Linux host (or WSL2)
- ARM cross-compiler toolchain (see `scripts/setup-toolchain.sh`)
- A Kobo Clara BW with developer mode enabled

### Quick Start

```bash
# Clone the repo
git clone https://github.com/your/project-aurora.git
cd project-aurora

# Set up the cross-compiler
./scripts/setup-toolchain.sh

# Build the dashboard backend
./scripts/build.sh

# Deploy to your Kobo
./scripts/deploy.sh
```

## Project Structure

```
project-aurora/
├── dashboard/          # Browser dashboard (Rust backend + web frontend)
├── services/           # Linux background services
├── packages/           # Package manager
├── widgets/            # FbInk widget framework
├── apps/               # Native Qt applications
├── nickel/             # Nickel reverse engineering & hooks
├── sdk/                # Plugin SDK
├── scripts/            # Build, deploy, and debug scripts
└── docs/               # Documentation
```

## License

GNU General Public License v3.0. See [LICENSE](LICENSE).
