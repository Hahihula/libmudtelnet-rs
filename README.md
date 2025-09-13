```
    ∞
   ╱ ╲    "The monad, of which we shall speak here,
  │   │    is nothing but a simple substance..."
  │ ● │
  │   │    libmudtelnet-rs - composable Telnet atoms
   ╲ ╱     for robust MUD clients
    ∞
```

# libmudtelnet-rs

**A robust, event-driven Telnet parser for MUD clients in Rust**

Low-level, RFC 854-correct, zero-allocation hot paths with comprehensive MUD protocol support.

## Standing on the Shoulders of Giants

libmudtelnet-rs is a fork of [libmudtelnet] by the [Blightmud] team, which powers the [Blightmud MUD client]. The original libmudtelnet was itself forked from [libtelnet-rs] by [envis10n], which is inspired by the robust C library [libtelnet] by [Sean Middleditch]. We are deeply grateful for their foundational work.

This project continues the specialization for the unique and demanding world of MUDs. Our focus is on adding comprehensive MUD protocol support, fixing critical edge-case bugs encountered in the wild, and relentlessly pursuing the correctness and performance that modern MUD clients deserve.

## Why MUD Clients Need This

You're building a MUD client. You need to handle Telnet negotiation, GMCP data streams, MCCP compression, and MSDP variables. Your parser must be robust against malformed sequences from decades-old servers while maintaining zero-allocation performance in hot paths.

Standard Telnet parsers expect compliance; MUD servers offer chaos. A stray `SE` byte without `IAC`, truncated subnegotiations, or multiple escaped `IAC` sequences can crash naive implementations. libmudtelnet handles these realities gracefully while delivering clean, structured events for your application logic.

Use libmudtelnet-rs so you can focus on building amazing triggers, mappers, and UIs instead of debugging protocol edge cases.

## Complete Protocol Support

### MUD-Specific Protocols (All Fully Supported)

1. **GMCP (201)** - Generic MUD Communication Protocol
   - Most widely adopted for JSON game data exchange
   - Complete negotiation and subnegotiation with payloads
   - Clean event delivery for your JSON parser

2. **MSDP (69)** - MUD Server Data Protocol
   - Structured data with VAR/VAL/TABLE/ARRAY tags
   - Complete tag definitions for robust parsing
   - State tracking for complex data structures

3. **MCCP2/MCCP3 (86/87)** - MUD Client Compression Protocol
   - Compression negotiation with proper signaling
   - Special `DecompressImmediate` events for boundary handling
   - Supports both server-to-client and bidirectional compression

4. **MXP (91)** - MUD eXtension Protocol
   - Markup and hyperlink protocol negotiation
   - Complete subnegotiation data delivery

5. **MSSP (70)** - MUD Server Status Protocol
   - Server information and capability exchange
   - Structured data parsing support

6. **ZMP (93)** - Zenith MUD Protocol
   - Package and module system negotiation
   - Extensible protocol framework support

7. **ATCP (200)** - Achaea Telnet Client Protocol
   - Legacy IRE MUD protocol support
   - Backward compatibility for older systems

### Standard Telnet Options (All Negotiable)

- **NAWS (31)** - Negotiate About Window Size
- **TTYPE (24)** - Terminal Type negotiation
- **CHARSET (42)** - Character set negotiation (RFC 2066)
- **ECHO (1)** - Echo control
- **SGA (3)** - Suppress Go Ahead
- **BINARY (0)** - Binary transmission mode
- **EOR (25)** - End of Record markers
- **TSPEED (32)** - Terminal speed negotiation
- **ENVIRON/NEWENVIRON (36/39)** - Environment variables
- **LINEMODE (34)** - Line-at-a-time input mode
- Plus 30+ additional standard options with full state tracking

## Usage

### Basic Parser Loop

```rust
use libmudtelnet_rs::{Parser, events::TelnetEvents};
use libmudtelnet_rs::telnet::{op_option, msdp};

// Create parser - place near your socket
let mut parser = Parser::new();

// Process incoming bytes from socket
let events = parser.receive(&socket_bytes);

// Handle structured events
for event in events {
    match event {
        TelnetEvents::DataReceive(data) => {
            // Raw text/data from the MUD
            app.display_text(&data);
        },

        TelnetEvents::Subnegotiation(sub) => {
            // Complete protocol payloads
            match sub.option {
                op_option::GMCP => {
                    // Parse GMCP JSON: {"room.info": {"name": "Temple"}}
                    let gmcp_data: serde_json::Value =
                        serde_json::from_slice(&sub.buffer)?;
                    app.handle_gmcp(gmcp_data);
                },
                op_option::MSDP => {
                    // Parse MSDP VAR/VAL pairs
                    app.parse_msdp_data(&sub.buffer);
                },
                op_option::MSSP => {
                    // Server status information
                    app.update_server_info(&sub.buffer);
                },
                _ => {}
            }
        },

        TelnetEvents::Negotiation(neg) => {
            // Automatic option negotiation (state tracked)
            app.log_negotiation(neg.command, neg.option);
        },

        TelnetEvents::DecompressImmediate(data) => {
            // MCCP2/3: Decompress this data before processing
            let decompressed = decompress(&data)?;
            let new_events = parser.receive(&decompressed);
            // Process new_events...
        },

        TelnetEvents::DataSend(data) => {
            // Send this data to the socket
            socket.write_all(&data)?;
        },

        _ => {}
    }
}

// Send text with proper escaping
if let Some(data) = parser.send_text("say Hello, world!\r\n") {
    socket.write_all(&data)?;
}
```

### Protocol Negotiation

```rust
use libmudtelnet_rs::events::TelnetNegotiation;
use libmudtelnet_rs::telnet::op_command::{WILL, DO};
use libmudtelnet_rs::telnet::op_option::{GMCP, NAWS};

// Enable GMCP support
let negotiation = TelnetNegotiation::new(WILL, GMCP);
if let Some(data) = parser.negotiate(&negotiation) {
    socket.write_all(&data)?;
}

// Request server's window size capability
let negotiation = TelnetNegotiation::new(DO, NAWS);
if let Some(data) = parser.negotiate(&negotiation) {
    socket.write_all(&data)?;
}
```

### MSDP Data Handling

```rust
use libmudtelnet_rs::telnet::msdp;

fn parse_msdp_data(data: &[u8]) {
    let mut i = 0;
    while i < data.len() {
        if data[i] == msdp::VAR {
            // Extract variable name
        } else if data[i] == msdp::VAL {
            // Extract variable value
        } else if data[i] == msdp::TABLE_OPEN {
            // Begin table parsing
        }
        // ... handle ARRAY_OPEN, TABLE_CLOSE, ARRAY_CLOSE
        i += 1;
    }
}
```

## Event-Driven Architecture

libmudtelnet-rs transforms the Telnet byte stream into a clean sequence of structured events:

- **DataReceive**: Text and data from the MUD server
- **DataSend**: Bytes your application must write to the socket
- **Negotiation**: WILL/WONT/DO/DONT option negotiations
- **Subnegotiation**: Protocol payloads (GMCP, MSDP, etc.)
- **IAC**: Low-level Telnet commands
- **DecompressImmediate**: MCCP compression boundary signals

This separation keeps your network I/O simple and your protocol handling clean.

## Battle-Tested Robustness

### Edge Case Handling

libmudtelnet-rs has been hardened against real-world protocol violations:

- **Unescaped SE bytes**: A `SE` byte without preceding `IAC` during subnegotiation is handled gracefully
- **Truncated subnegotiations**: Malformed sequences like `IAC SB IAC SE` won't cause panics
- **Multiple IAC escaping**: Complex escape sequences (`IAC IAC IAC IAC`) are correctly unescaped
- **Option 0xFF handling**: Negotiation of option 255 with truncated data is handled safely

### Testing and Validation

- **Fuzz Testing**: Continuous fuzzing with `cargo-fuzz` bombards the parser with malformed inputs
- **Compatibility Tests**: Validates behavior against libtelnet-rs test cases
- **Edge Case Tests**: Specific tests for each documented bug fix
- **Property Tests**: Round-trip and invariant validation
- **Production Heritage**: Serves as the foundation for Blightmud's Telnet implementation

## Performance and Memory

- **Zero-allocation hot paths**: Uses `bytes::BytesMut` to avoid copies in parsing loops
- **Zero-copy events**: Protocol payloads delivered as `bytes::Bytes` slices
- **Efficient state tracking**: Minimal memory footprint for negotiation states
- **no_std compatible**: Works in embedded environments (disable default features)

## API Stability

libmudtelnet-rs maintains API compatibility with libtelnet-rs where practical. The event semantics are stable - breaking changes follow semver and include migration guides.

## Contributing

We welcome contributions from the MUD and Rust communities! Whether you've found a bug, want to add protocol support, or improve documentation, your help is appreciated.

### Getting Started

```bash
# Build and test
cargo build
cargo test
cargo test --no-default-features  # Test no_std compatibility

# Code quality
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings

# Fuzz testing (requires cargo-fuzz)
cargo install cargo-fuzz
cd fuzz && cargo fuzz run parser_receive

# Benchmarks
cd compat && cargo bench
```

### Ways to Contribute

- **Report Bugs**: Found a MUD server that sends data the parser doesn't handle? Please [open an issue](https://github.com/blightmud/libmudtelnet/issues) with details
- **Add Protocol Support**: Want support for a new MUD protocol? Let's discuss implementation approach
- **Improve Tests**: Additional fuzz targets, edge cases, or property tests are always valuable
- **Documentation**: Code examples, protocol explanations, or usage guides

This project follows the Rust Code of Conduct. We're committed to providing a welcoming environment for all contributors.

## Compatibility

libmudtelnet-rs has been tested for API compatibility with libtelnet-rs. While much of the implementation has been rewritten for improved correctness and performance, the public API remains familiar to ease migration.

See [CHANGELOG.md](CHANGELOG.md) for detailed information about fixes and enhancements.

## Credits

Many thanks to:

* The [Blightmud] team for their work on [libmudtelnet], which `libmudtelnet-rs` is forked from.
* [envis10n] for his work on [libtelnet-rs], which `libmudtelnet` was originally forked from.
* [Sean Middleditch] for his work on [libtelnet], which inspired `libtelnet-rs`.

[github-actions-badge]: https://img.shields.io/github/actions/workflow/status/blightmud/libmudtelnet/rust.yml?label=build%20%26%20tests&logo=github&style=for-the-badge&branch=main
[github-actions-link]: https://github.com/blightmud/libmudtelnet/actions/workflows/rust.yml?query=branch%3Amain
[crate-badge]: https://img.shields.io/crates/v/libmudtelnet.svg?style=for-the-badge
[crate-link]: https://crates.io/crates/libmudtelnet
[docs-badge]: https://img.shields.io/badge/docs-rs-blue.svg?style=for-the-badge
[docs-link]: https://docs.rs/libmudtelnet
[libtelnet-rs]: https://github.com/envis10n/libtelnet-rs
[libtelnet]: https://github.com/seanmiddleditch/libtelnet
[libmudtelnet]: https://github.com/Blightmud/libmudtelnet
[Blightmud]: https://github.com/Blightmud/Blightmud
[Blightmud MUD client]: https://github.com/Blightmud/Blightmud
[envis10n]: https://github.com/envis10n/
[Sean Middleditch]: https://github.com/seanmiddleditch/
