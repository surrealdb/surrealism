# Surrealism

WebAssembly module system for SurrealDB. Surrealism allows you to build custom functions and modules for SurrealDB using WebAssembly.

## Overview

Surrealism provides a complete framework for building, testing, and deploying WebAssembly modules that extend SurrealDB's functionality. Modules can be written in any language that compiles to WebAssembly, including Rust, Go, AssemblyScript, and more.

## Crates

This repository contains the following crates:

- **surrealism**: Main API for building WASM modules
- **surrealism-runtime**: Host-side runtime for executing WASM modules
- **surrealism-types**: Language-agnostic serialization framework for WASM guest-host communication
- **surrealism-macros**: Procedural macros for deriving traits
- **surrealism-cli**: Command-line tool for building and managing WASM modules
- **demo**: Example WASM module implementation

## Documentation

For detailed documentation, see the [surrealism-types README](surrealism-types/README.md) for information about the serialization protocol and architecture.

## License

This project is licensed under the Business Source License 1.1. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please ensure that your changes maintain compatibility with the existing SurrealDB integration.

