# Rust SSH Demo

This is a simple project to learn how to interact with the Raspberry PI/VPS through SSH using a Rust application.

## Available Commands

- `test`
- `info hardware`
- `info memory`
- `info partitions`
- `info temperature`
- `file upload {}`
- `file download {}`
- `action reboot`
- `action shutdown`

## Optional Arguments

You can pass the following optional arguments to the application:

- `--host` - The host to connect to
- `--port` - The port to connect to
- `--user` - The user to connect as

By default the application will use the values defined in the `.env` file.
