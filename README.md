# Rust SSH Demo

This is a simple project to learn how to interact with the Raspberry PI/VPS through SSH using a Rust application.

## Available Commands

Usage: `-- [OPTIONS] [COMMAND]`

- `test`
- `info hardware`
- `info memory`
- `info partitions`
- `info cpu`
- `info temperature`
- `info uptime`
- `info version`
- `info current-dir`
- `file upload {file on local machine} {destination on remote machine}`
- `file download {file on remote machine} {destination on local machine}`
- `action reboot`
- `action shutdown`

Example: `-- --user=pi --host=192.168.178.225 info temperature`

## Optional Arguments

You can pass the following optional arguments to the application:

- `--host` - The host to connect to
- `--port` - The port to connect to
- `--user` - The user to connect as

By default the application will use the values defined in the `.env` file.

## Help

You can get help by running the application with the `--help` or `-h` flag.
