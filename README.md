# Groundhog AI Coding Assistant

Groundhog's primary purpose is to teach people how Cursor and all these other coding agents work under the hood. If you understand how these coding assistants work from first principles, then you can drive these tools harder (or perhaps make your own!). As part of the series kicked off at http://ghuntley.com/specs we'll be building it together, increment by increment.

Please don't raise GitHub issues mentioning that XYZ does not work as I'm yet to decide on the community model around the project and doing customer support for free is not high up on my list. 

_Groundhog is a teaching tool first_. If you want a full-blown thing right now, go check out "Goose", "Roo/Cline", "Aider" or "AllHands".

![](./assets/groundhog.png)

## Features

- **Code Explanation**: Get detailed explanations of code snippets and files
- **Modern Architecture**: Built with Rust for performance and reliability
- **Comprehensive Logging**: Built-in logging and telemetry for debugging and monitoring
- **CLI Interface**: Easy-to-use command-line interface

## Installation

[Installation instructions to be added]

## Usage

The basic command structure is:

```bash
Groundhog <command> [options]
```

### Available Commands

- `explain`: Get explanations for code snippets or files
  ```bash
  Groundhog explain
  ```

More commands will be added in future releases.

## Development

### Prerequisites

- Rust toolchain
- [Other prerequisites to be added]

### Building from Source

```bash
git clone [repository-url]
cd Groundhog
cargo build
```

### Running Tests

```bash
cargo test
```

## Documentation

Detailed documentation is available in the `specs/` directory:

- [Architecture](specs/architecture.md)
- [CLI Interface](specs/cli_interface.md)
- [Logging & Telemetry](specs/logging_telemetry.md)
- [Commands](specs/commands.md)

## Contributing

[Contribution guidelines to be added]

## License

[License information to be added] 