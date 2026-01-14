# GOTO

Quick navigation to web resources from your terminal.

## Install

Requires `libssl-dev`.

```bash
make build && sudo make install
```

To uninstall:
```bash
sudo make uninstall
```

## Usage

```bash
goto github                      # Open GitHub page for current repo
goto github -c <hash>            # Open specific commit
goto travis                      # Open Travis CI page
goto rust -s <term>              # Search Rust std docs

goto url                         # List configured URLs
goto url <key>                   # Open a saved URL
goto config url <key> <url>      # Save a URL shortcut
```

Run `goto --help` for more details.
