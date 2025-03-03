# Groundhog AI Coding Assistant Specifications

This document provides an overview of the Groundhog AI coding assistant specifications. Each domain topic has its own detailed specification file in the `specs/` directory.

## Specification Documents

| Topic | Description | Link |
|-------|-------------|------|
| Architecture | Overall system architecture and design decisions | [Architecture](specs/architecture.md) |
| CLI Interface | Command-line interface specifications | [CLI Interface](specs/cli_interface.md) |
| Logging & Telemetry | Logging, metrics, and telemetry specifications | [Logging & Telemetry](specs/logging_telemetry.md) |
| Commands | Individual command specifications | [Commands](specs/commands.md) |

## Overview

Groundhog is an AI-powered coding assistant that helps developers with various coding tasks. The application is built in Rust and uses modern practices for logging, metrics, and telemetry.

## Getting Started

To use Groundhog, you can run various commands starting with `Groundhog`. The first implemented command is:

```bash
Groundhog explain
```

This command currently prints "hello world" as a basic implementation. 