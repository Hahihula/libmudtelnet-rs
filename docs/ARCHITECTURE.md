# libmudtelnet Architecture Guide

## Overview

libmudtelnet is a low-level Telnet protocol implementation designed specifically for MUD (Multi-User Dungeon) clients. The architecture emphasizes **zero-copy efficiency**, **robust state management**, and **compatibility** with the original libtelnet implementations.

## Core Design Principles

### 1. **Zero-Copy Data Processing**
The library minimizes memory allocations and data copying through strategic use of the `bytes` crate:

- **Bytes and BytesMut**: Immutable and mutable byte containers with reference counting
- **Slicing**: O(1) operations to create views into existing data without copying
- **Split/Freeze Pattern**: Convert mutable buffers to immutable ones efficiently

```rust
// Zero-copy slicing in extract_event_data()
let buf = self.buffer.split().freeze();  // O(1) - no copy
events.push(EventType::None(buf.slice(cmd_begin..index)));  // O(1) view
```

### 2. **State Machine Architecture**
Multiple clean state machines handle different aspects of Telnet parsing:

#### Main Parser State Machine (`extract_event_data`)
```
Normal -> IAC -> Negotiation/Subnegotiation/IAC Command
       -> Sub -> SubOpt -> SubIac -> (back to Normal or MCCP handling)
```

#### IAC Escape/Unescape State Machines
Separate state machines for handling IAC byte doubling/undoubling with clear state transitions.

### 3. **Separation of Concerns**
The codebase is cleanly modularized:

```
src/
├── lib.rs           # Core Parser and main processing logic
├── events.rs        # Event types and serialization
├── telnet.rs        # Protocol constants (commands, options)  
├── compatibility.rs # Option negotiation state management
└── tests/           # Comprehensive unit tests
```

## Module Architecture

### Parser Core (`src/lib.rs`)

The `Parser` struct is the main entry point with clear responsibilities:

```rust
pub struct Parser {
    pub options: CompatibilityTable,  // Negotiation state
    buffer: BytesMut,                 // Accumulation buffer
}
```

**Key Methods**:
- `receive(&mut self, data: &[u8])` - Main entry point for data processing
- `process(&mut self)` - Internal event extraction and processing  
- `extract_event_data(&mut self)` - State machine for parsing protocol
- `escape_iac/unescape_iac` - Static utilities for IAC handling

**Design Strengths**:
- Stateful but not overly complex
- Clear separation between public API and internal processing
- Buffer management isolated from protocol logic

### Events System (`src/events.rs`)

Clean event-driven architecture with typed protocol elements:

```rust
pub enum TelnetEvents {
    IAC(TelnetIAC),                    // 2-byte IAC sequences
    Negotiation(TelnetNegotiation),    // 3-byte negotiations  
    Subnegotiation(TelnetSubnegotiation), // Variable-length subs
    DataReceive(Bytes),                // Incoming text data
    DataSend(Bytes),                   // Outgoing protocol data
    DecompressImmediate(Bytes),        // MCCP compressed data
}
```

**Design Strengths**:
- Each event type is self-contained with its own serialization
- Clear distinction between incoming data and outgoing protocol responses
- Type safety prevents mixing different protocol elements

### Compatibility Management (`src/compatibility.rs`)

Sophisticated but simple option negotiation tracking:

```rust
pub struct CompatibilityTable {
    options: [u8; 256],  // Bit-packed state for all 256 possible options
}

pub struct CompatibilityEntry {
    pub local: bool,        // Do we support this option locally?
    pub remote: bool,       // Do we support this option remotely?  
    pub local_state: bool,  // Is this option currently enabled locally?
    pub remote_state: bool, // Is this option currently enabled remotely?
}
```

**Design Strengths**:
- Compact representation (256 bytes for all options)
- Clear separation between capability and current state
- Bit manipulation abstracted behind clean API

### Protocol Constants (`src/telnet.rs`)

Simple, clear constant definitions using macros for consistency:

```rust
macro_rules! u8_const {
    ($name: ident, $value: expr) => {
        pub const $name: u8 = $value;
    };
}
```

**Design Strengths**:
- Single source of truth for protocol constants
- Consistent naming and typing
- Modular organization (commands vs options)

## Data Flow Architecture

### Incoming Data Processing

```
Raw bytes -> Parser::receive() -> extract_event_data() -> process() -> Vec<TelnetEvents>
                     ↑                    ↓
                Buffer accumulation    State machine parsing
                                          ↓  
                                   Event classification
                                          ↓
                                   Option state updates
```

### Event Processing Flow

```rust
// Simplified processing flow
fn process(&mut self) -> Vec<TelnetEvents> {
    let events = self.extract_event_data();  // Parse buffer into typed events
    let mut event_list = Vec::new();
    
    for event in events {
        match event {
            EventType::None(buffer) => /* Handle data */,
            EventType::Neg(buffer) => /* Handle negotiations with state changes */,
            EventType::SubNegotiation(buffer, remaining) => /* Handle subnegotiations */,
            // ...
        }
    }
    event_list
}
```

### Option Negotiation Flow

```
Incoming WILL/WONT/DO/DONT -> process_negotiation()
                                     ↓
                            Check CompatibilityTable  
                                     ↓
                            Update state if valid
                                     ↓
                            Generate response events
```

## Memory Management Strategy

### Buffer Management
- **Single accumulation buffer**: `BytesMut` in the Parser
- **Zero-copy slicing**: Create views without copying data
- **Automatic cleanup**: Buffer is consumed during processing

### Allocation Patterns
- **Minimal allocations**: Only allocate when necessary for protocol responses
- **Capacity hints**: Pre-size buffers when final size is known
- **Reuse opportunities**: Event vectors pre-allocated with reasonable capacity

### Safety Guarantees
- **No unsafe code**: Pure safe Rust throughout
- **Bounds checking**: All buffer access is bounds-checked
- **Reference counting**: Bytes handles cleanup automatically

## Protocol Handling Design

### IAC (Interpret As Command) Processing

The library correctly handles the complex IAC escape sequences:

```rust
// IAC byte (255) handling:
// - Single IAC + command -> IAC event
// - Double IAC (255, 255) -> Escaped literal 255 in data  
// - IAC + negotiation -> Negotiation event
// - IAC SB ... IAC SE -> Subnegotiation event
```

**State Machine Approach**:
- Clear states for each parsing context
- Proper handling of incomplete sequences
- Robust recovery from malformed input

### Subnegotiation Parsing

Complex subnegotiation handling with proper IAC escaping:

```rust
// Subnegotiation format: IAC SB <option> <data> IAC SE
// <data> may contain escaped IAC bytes (doubled)
// Empty subnegotiations (just IAC SB <option> IAC SE) are valid
```

### MCCP (MUD Client Compression Protocol) Support

Special handling for compression protocols:

```rust
// MCCP2/MCCP3 subnegotiations signal start of compressed data
// All data after the subnegotiation is compressed
// DecompressImmediate event signals client must decompress before further parsing
```

## Testing Strategy

### Multi-Level Testing Approach

1. **Unit Tests** (`tests/tests.rs`):
   - Protocol parsing validation
   - State machine behavior verification
   - Event generation testing

2. **Fuzzing** (`fuzz/`):
   - Property-based testing with `libfuzzer-sys`
   - Comparison testing against reference implementation
   - Edge case discovery through randomized inputs

3. **Compatibility Testing** (`compat/`):
   - Direct comparison with original `libtelnet-rs` 
   - Behavioral parity validation
   - Performance benchmarking

### Testing Philosophy
- **Differential testing**: Validate against known-good implementation
- **Property-based**: Ensure escape/unescape operations are inverse
- **State verification**: Validate option negotiation state consistency

## Performance Characteristics

### Time Complexity
- **Parsing**: O(n) where n is input size - single pass through data
- **Event generation**: O(k) where k is number of events - minimal processing per event
- **Option lookup**: O(1) - array access by option number
- **Buffer operations**: O(1) for slicing, O(n) only when copying is required

### Space Complexity
- **Parser state**: O(1) - fixed size regardless of input
- **Option table**: O(1) - exactly 256 bytes
- **Event storage**: O(k) where k is number of events generated
- **Buffer growth**: Bounded by input size, cleared after processing

### Optimization Patterns
- **Lazy allocation**: Only allocate when escape sequences are present
- **Batch processing**: Process entire buffers at once rather than byte-by-byte
- **State reuse**: Maintain parser state across calls to avoid reinitialization

## Security Considerations

### Input Validation
- **Bounds checking**: All buffer access is safe
- **State validation**: Invalid state transitions are handled gracefully
- **Resource limits**: No unbounded resource consumption

### Protocol Safety
- **IAC escaping**: Proper handling prevents injection attacks
- **State isolation**: Option state cannot be corrupted by malformed input
- **Buffer management**: No buffer overflows or underflows possible

## Extension Points

### Adding New Options
```rust
// Add constants in telnet.rs
pub mod op_option {
    u8_const!(NEW_OPTION, 123);
}

// Add support in compatibility table
compatibility.support_local(op_option::NEW_OPTION);

// Handle in subnegotiation processing
match option {
    op_option::NEW_OPTION => handle_new_option(&buffer),
    // ...
}
```

### Custom Event Types
The event system can be extended while maintaining backward compatibility through additional enum variants.

### Protocol Extensions
New IAC commands can be added to the constants and handled in the main state machine.

## Anti-Patterns Avoided

### Common Pitfalls Not Present
- **String parsing**: No string-based protocol parsing (uses bytes throughout)
- **Recursive parsing**: Flat state machine avoids stack overflow  
- **Blocking operations**: No I/O operations in parsing logic
- **Global state**: Parser state is encapsulated and thread-safe
- **Memory leaks**: Reference counting handles cleanup automatically

### Performance Anti-Patterns Avoided  
- **Excessive copying**: Zero-copy design throughout
- **Repeated allocations**: Pre-sized allocations and reuse patterns
- **N+1 queries**: Batch processing avoids per-byte overhead
- **String concatenation**: Binary processing avoids string overhead

## Future Evolution

The architecture supports clean evolution:
- **API extensions**: Event types can be added without breaking changes
- **Performance improvements**: Buffer management can be optimized internally
- **Protocol support**: New Telnet options can be added modularly
- **Feature flags**: Optional functionality can be feature-gated

The codebase demonstrates that robust protocol handling doesn't require complex architecture - clear separation of concerns, proper state management, and zero-copy design principles create a maintainable and efficient implementation.