# CLAUDE.md

## Project Overview

GOTO is a Rust CLI utility for quick navigation to web resources from the terminal. It opens URLs in the default browser with special support for GitHub repositories, Travis CI, and Rust documentation.

## Build Commands

```bash
make build          # Lint, test, and build release binary
make test           # Run unit tests
make lint           # Run clippy and format check
make fmt            # Auto-format code
sudo make install   # Install to /usr/local/bin
sudo make uninstall # Remove from /usr/local/bin
```

## Project Structure

- `src/main.rs` - Single-file source containing all logic and tests
- `config/default_urls.json` - Example URL mappings
- User config: `~/.config/goto/urls.json`

## Key Dependencies

- `structopt` - CLI argument parsing with subcommands
- `git2` - Git repository access
- `serde_json` - JSON config parsing
- `quicli` - CLI framework

## CLI Commands

| Command | Description |
|---------|-------------|
| `goto github` | Opens GitHub repo for current directory |
| `goto github -c <hash>` | Opens specific commit on GitHub |
| `goto travis` | Opens Travis CI page for repo |
| `goto rust -s <term>` | Searches Rust std docs |
| `goto url` | Lists all configured URL shortcuts |
| `goto url <key>` | Opens custom URL from config |
| `goto config url <key> <url>` | Adds or updates a URL shortcut |

## Code Patterns

- Uses `xdg-open` to launch browser (Linux)
- Git SSH URLs converted to HTTPS via `parse_git_origin_to_github_url()`
- Error handling via `failure` crate with custom error types
- Config stored as JSON with `url_map` object
- Tests in `#[cfg(test)] mod tests` at end of main.rs

## System Requirements

- libssl-dev (OpenSSL development library)
- clippy and rustfmt for linting
