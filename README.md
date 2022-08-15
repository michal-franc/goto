## GOTO

This is a small cli app used to `goto` things.

If you ever wanted to open your default browser with a link to your project github page from the `command line` inside your `github repo folder`. This is the tool for you.

At the moment it supports 3 commands:

`github` 
- opens up browser with `remote origin` and `master` branch url

`github -c <hash>` 
- opens up browser to specific commit in github

`travis` 
- open up browser with travis pointing to `origin` of your repo in the `master` branch

`rust -s arg`
- open up browser with rust std docs using `arg` as a param to search for

`<any other name>`
- looks for the key in ~/.config/goto/urls.json and tries to open website if key exist

## Install

To install - by default it moves the binary to `/usr/local/bin`
```
make build && sudo make install
```

To uninstall

```
sudo make uninstall
```
