# Rust SSH Demo

This is a simple project to learn how to interact with the Raspberry PI/VPS through SSH using a Rust application.

## Available Commands

Usage: `[OPTIONS] [COMMAND]`

### Test

- `test`

### Action

- `action reboot`
- `action shutdown`

### Control

- `control update`

### File

- `file upload {file} {destination}`
- `file download {file} {destination}`
- `action view {file}`

### Info

- `info hardware`
- `info memory`
- `info partitions`
- `info cpu`
- `info temperature`
- `info uptime`
- `info version`
- `info pwd`
- `info du {directory}`

### Install

- `install nginx`

### Development

Example: `cargo run -- --user=pi --host=192.168.178.225 info temperature`

### Production

Example `.\rust-ssh-demo.exe --public-key=../../.ssh/id_rsa.pub --private-key=../../.ssh/id_rsa info temperature`

## Optional Arguments

You can pass the following optional arguments to the application:

- `--host` - The host to connect to
- `--port` - The port to connect to
- `--user` - The user to connect as
- `--public-key` - The path to the public key file
- `--private-key` - The path to the private key file

By default the application will use the values defined in the `.env` file.

## Help

You can get help by running the application with the `--help` or `-h` flag.
