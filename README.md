# aas-rs - Asset Administration Shell for Rust

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-2024-orange)](https://www.rust-lang.org/)
[![Cargo Package](https://img.shields.io/crates/v/aas.svg)](https://crates.io/crates/aas)

A comprehensive Rust implementation of the **Asset Administration Shell (AAS) Specification** with full support for JSON, XML, and advanced Web APIs.

## Overview

**aas-rs** is a workspace containing multiple specialized crates that together provide a production-ready implementation of the AAS standard:

- **aas** - Core type definitions for all AAS specifications with complete type safety
- **aas-json** - JSON serialization/deserialization
- **aas-xml** - XML serialization/deserialization  
- **aas-bench** - Performance benchmarks with Criterion and Flamegraph analysis

## Features

### Type Safety and Validation

- Strict type safety leveraging Rust's type system
- Automatic validation of formats and constraints
- Parsing of standardized data types (ISO8601, decimal numbers, URIs, language tags)
- Compile-time guarantees for API correctness
- Constraint enforcement through type design

### Multi-Format Support

- **JSON** - Complete serialization/deserialization with serde
- **XML** - Native XML support via quick-xml
- **OpenAPI** - Automatic API documentation generation (feature-gated)
- **Axum Integration** - Generic REST Web API framework integration
- **Format Flexibility** - Support for multiple serialization variants

### Comprehensive Specifications

- Part 1 (Basic Concepts) of the AAS specification
- Part 2 (Submodels and APIs) implementation

### Performance and Benchmarking

- Criterion-based benchmarks with HTML reports
- Flamegraph profiling for optimization
- SIMD-JSON support for high-speed parsing
- Memory-efficient type representations

### Developer Experience

- Comprehensive API documentation
- Code examples for common use cases
- Clear error messages with context
- Zero-cost abstractions

## Features

### Feature Flags

- `json` - JSON serialization support (default)
- `xml` - XML serialization support
- `openapi` - OpenAPI specification generation
- `part2` - Part 2 implementation with Axum web framework

## Roadmap

FEATURES:
- AASX file format support (ZIP-based container)
- RDF/OWL serialization format
- JSON Schema validation

IMPROVEMENTS:
- Refactored module structure for better organization
- Improved error messages with better diagnostics
- Performance optimization with SIMD
- Memory profiling and optimization

### Version 1.0.0 (Target)

MILESTONES:
- Full compliance with official AAS specification
- Production-grade API stability
- Comprehensive documentation and examples
- 95%+ test coverage
- Performance benchmarks for all operations

FEATURES:
- AASX signing and validation

## Testing

### Run All Tests

```bash
# Standard test suite
cargo test

# With all features enabled
cargo test --all-features

# Specific crate tests
cargo test -p aas
cargo test -p aas-json
cargo test -p aas-xml

# With output
cargo test -- --nocapture

# Single threaded execution
cargo test -- --test-threads=1
```

### Run Benchmarks

```bash
# Run Criterion benchmarks
cargo bench -p aas-bench

# Generate HTML report
cargo bench -p aas-bench -- --verbose

# View previous reports
# Open target/criterion/report/index.html
```

### Integration Tests

```bash
# Run with specific test files
cargo test --test '*' -- --include-ignored

# Profile with flamegraph (requires flamegraph installed)
cargo flamegraph --bench benchmark
```

## Documentation

### Generate API Documentation

```bash
cargo doc --no-deps --open
```

### Feature-Specific Documentation

```bash
# With OpenAPI features
cargo doc --no-deps --features "openapi" --open

# With all features
cargo doc --no-deps --all-features --open
```

<!-- ### OpenAPI Specification

Enable the `openapi` feature to automatically generate an OpenAPI 3.0 specification:

```rust
#[cfg(feature = "openapi")]
use aas::openapi::OpenApiSpec;

let spec = OpenApiSpec::generate();
println!("{}", serde_json::to_string_pretty(&spec)?);
``` -->

## Performance Characteristics

- Zero-copy where possible (TODO)
- Memory-efficient type layouts
- Streaming JSON parsing support
- Lazy deserialization capabilities
- Optimized constraint checking

## Contributing

Contributions are welcome and appreciated!

### High-Priority Contribution Areas

- Unit and integration tests for all components
- Documentation and code comments
- Bug fixes and code improvements
- Performance optimizations
- Additional examples and use cases
- Support for new AAS specification versions

## License

This project is licensed under the MIT License - see [LICENSE](./LICENSE) file for details.

## Author

**Fynn Kleine** - code@onlyfynns.de

## Resources

- Asset Administration Shell Reference: https://www.plattform-i40.de/
- Rust Official Documentation: https://doc.rust-lang.org/
- Serde Serialization: https://serde.rs/
- OpenAPI Specification: https://spec.openapis.org/
- Axum Web Framework: https://github.com/tokio-rs/axum

If you find this project helpful, please consider giving it a star on GitHub!