# libmudtelnet API Examples

This document provides comprehensive examples of using libmudtelnet in real-world scenarios. The library excels at handling Telnet protocol parsing for MUD (Multi-User Dungeon) clients with zero-copy efficiency and robust state management.

## Basic Setup and Data Processing

### Simple Parser Setup
```rust
use libmudtelnet::{Parser, events::TelnetEvents};
use libmudtelnet::telnet::{op_option, op_command};

// Create a parser with default options
let mut parser = Parser::new();

// Or create with specific buffer capacity
let mut parser = Parser::with_capacity(1024);

// Process incoming data from a socket
let data = b"Hello, world!\r\n";
let events = parser.receive(data);

for event in events {
    match event {
        TelnetEvents::DataReceive(data) => {
            println!("Text: {}", String::from_utf8_lossy(&data));
        }
        TelnetEvents::DataSend(data) => {
            // Send this data back to the remote end
            // socket.write_all(&data)?;
        }
        _ => {} // Handle other event types as needed
    }
}
```

### Handling All Event Types
```rust
use libmudtelnet::{Parser, events::TelnetEvents};

fn handle_telnet_events(events: Vec<TelnetEvents>) {
    for event in events {
        match event {
            TelnetEvents::DataReceive(data) => {
                // Regular text data from the MUD server
                print!("{}", String::from_utf8_lossy(&data));
            }
            
            TelnetEvents::DataSend(data) => {
                // Data that needs to be sent back to server
                // This handles protocol negotiations automatically
                send_to_server(&data);
            }
            
            TelnetEvents::IAC(iac) => {
                // IAC commands like Go Ahead (GA) or End of Record (EOR)  
                println!("IAC command: {}", iac.command);
                if iac.command == op_command::GA {
                    println!("Server sent Go Ahead - ready for input");
                }
            }
            
            TelnetEvents::Negotiation(neg) => {
                // Option negotiations (WILL/WONT/DO/DONT)
                println!("Negotiation: {} for option {}", 
                    command_name(neg.command), option_name(neg.option));
            }
            
            TelnetEvents::Subnegotiation(sub) => {
                // Subnegotiation data (like GMCP, NAWS, etc.)
                handle_subnegotiation(sub.option, &sub.buffer);
            }
            
            TelnetEvents::DecompressImmediate(data) => {
                // MCCP2/MCCP3 compressed data - decompress before parsing
                let decompressed = decompress_mccp(&data);
                let new_events = parser.receive(&decompressed);
                handle_telnet_events(new_events); // Recursive processing
            }
        }
    }
}

fn send_to_server(data: &[u8]) {
    // Implementation depends on your networking approach
    // tcp_stream.write_all(data).unwrap();
}
```

## Option Support and Negotiation

### Setting Up Common MUD Options
```rust
use libmudtelnet::{Parser, compatibility::CompatibilityTable};
use libmudtelnet::telnet::op_option;

// Create parser with support for common MUD options
let mut compatibility = CompatibilityTable::new();

// Support GMCP (Generic MUD Communication Protocol)
compatibility.support_local(op_option::GMCP);

// Support NAWS (Negotiate About Window Size)  
compatibility.support_local(op_option::NAWS);

// Support terminal type negotiation
compatibility.support_local(op_option::TTYPE);

// Support both directions for some options
compatibility.support(op_option::ECHO);  // Both local and remote
compatibility.support(op_option::SGA);   // Suppress Go Ahead

let parser = Parser::with_support(compatibility);
```

### Manual Option Negotiation
```rust
// Indicate we want to use an option locally
if let Some(event) = parser._will(op_option::GMCP) {
    // Send this negotiation to the server
    if let TelnetEvents::DataSend(data) = event {
        send_to_server(&data);
    }
}

// Ask the server to use an option
if let Some(event) = parser._do(op_option::NAWS) {
    // Send this request to the server
    if let TelnetEvents::DataSend(data) = event {
        send_to_server(&data);
    }
}

// Check if linemode is enabled
if parser.linemode_enabled() {
    println!("Server is using linemode");
}
```

## Subnegotiation Examples

### GMCP (Generic MUD Communication Protocol)
```rust
use libmudtelnet::telnet::op_option;
use serde_json::{json, Value};

// Send GMCP data to the server
let gmcp_data = json!({
    "Core.Supports.Set": [
        "Char 1",
        "Char.Stats 1", 
        "Room 1"
    ]
});

let gmcp_string = gmcp_data.to_string();
if let Some(event) = parser.subnegotiation_text(op_option::GMCP, &gmcp_string) {
    if let TelnetEvents::DataSend(data) = event {
        send_to_server(&data);
    }
}

// Handle incoming GMCP subnegotiation
fn handle_subnegotiation(option: u8, buffer: &[u8]) {
    match option {
        op_option::GMCP => {
            let gmcp_str = String::from_utf8_lossy(buffer);
            if let Ok(value) = serde_json::from_str::<Value>(&gmcp_str) {
                handle_gmcp_message(value);
            }
        }
        op_option::NAWS => {
            handle_window_size(buffer);
        }
        _ => {
            println!("Unknown subnegotiation for option {}", option);
        }
    }
}

fn handle_gmcp_message(data: Value) {
    // Process GMCP data from the MUD
    println!("GMCP: {}", data);
}
```

### NAWS (Negotiate About Window Size)
```rust
use bytes::{BufMut, BytesMut};

// Send window size to server
fn send_window_size(parser: &mut Parser, width: u16, height: u16) {
    let mut naws_data = BytesMut::with_capacity(4);
    naws_data.put_u16(width);
    naws_data.put_u16(height);
    
    if let Some(event) = parser.subnegotiation(op_option::NAWS, naws_data.freeze()) {
        if let TelnetEvents::DataSend(data) = event {
            send_to_server(&data);
        }
    }
}

// Handle incoming NAWS data
fn handle_window_size(buffer: &[u8]) {
    if buffer.len() >= 4 {
        let width = u16::from_be_bytes([buffer[0], buffer[1]]);
        let height = u16::from_be_bytes([buffer[2], buffer[3]]);
        println!("Server requested window size: {}x{}", width, height);
    }
}
```

## Advanced Usage Patterns

### Processing Socket Data
```rust
use std::net::TcpStream;
use std::io::{Read, Write};

fn handle_mud_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut parser = setup_mud_parser();
    let mut buffer = [0u8; 4096];
    
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, // Connection closed
            Ok(n) => {
                let events = parser.receive(&buffer[..n]);
                
                for event in events {
                    match event {
                        TelnetEvents::DataReceive(data) => {
                            // Display text to user
                            print!("{}", String::from_utf8_lossy(&data));
                        }
                        TelnetEvents::DataSend(data) => {
                            // Send protocol response back to server
                            stream.write_all(&data)?;
                        }
                        TelnetEvents::Subnegotiation(sub) => {
                            handle_subnegotiation(sub.option, &sub.buffer);
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}

fn setup_mud_parser() -> Parser {
    let mut compatibility = CompatibilityTable::new();
    
    // Common MUD client options
    compatibility.support_local(op_option::GMCP);
    compatibility.support_local(op_option::NAWS);
    compatibility.support_local(op_option::TTYPE);
    compatibility.support(op_option::SGA);
    compatibility.support(op_option::ECHO);
    
    Parser::with_support(compatibility)
}
```

### Sending User Input
```rust
// Send user input with proper IAC escaping
fn send_user_input(parser: &mut Parser, input: &str) -> TelnetEvents {
    // The send_text method automatically escapes IAC bytes and adds \r\n
    parser.send_text(input)
}

// Usage
let user_input = "look";
let event = send_user_input(&mut parser, user_input);
if let TelnetEvents::DataSend(data) = event {
    stream.write_all(&data)?;
}
```

### Manual IAC Escaping
```rust
use libmudtelnet::Parser;

// If you need to send raw data that might contain IAC bytes
let raw_data = vec![255, 1, 2, 255, 3]; // Contains IAC (255) bytes
let escaped = Parser::escape_iac(raw_data);
// Result: [255, 255, 1, 2, 255, 255, 3] - IACs are doubled

// Reverse the escaping
let unescaped = Parser::unescape_iac(escaped);
// Back to: [255, 1, 2, 255, 3]
```

## Error Handling and Edge Cases

### Robust Event Processing
```rust
fn safe_event_processing(events: Vec<TelnetEvents>) {
    for event in events {
        match event {
            TelnetEvents::DataReceive(data) => {
                // Always validate text data for display
                match String::from_utf8(data.to_vec()) {
                    Ok(text) => print!("{}", text),
                    Err(_) => {
                        // Handle binary or invalid UTF-8 data
                        print!("{}", String::from_utf8_lossy(&data));
                    }
                }
            }
            
            TelnetEvents::Subnegotiation(sub) => {
                // Validate subnegotiation data length and content
                if sub.buffer.is_empty() {
                    // Empty subnegotiation - valid but no data
                    println!("Empty subnegotiation for option {}", sub.option);
                } else {
                    handle_subnegotiation(sub.option, &sub.buffer);
                }
            }
            
            TelnetEvents::DecompressImmediate(data) => {
                // Always decompress MCCP data before further processing
                match decompress_mccp_data(&data) {
                    Ok(decompressed) => {
                        let new_events = parser.receive(&decompressed);
                        safe_event_processing(new_events);
                    }
                    Err(e) => {
                        eprintln!("MCCP decompression failed: {}", e);
                    }
                }
            }
            
            _ => {} // Handle other events
        }
    }
}
```

### Connection State Management
```rust
use libmudtelnet::compatibility::CompatibilityEntry;

struct MudClient {
    parser: Parser,
    connected: bool,
    supported_options: Vec<u8>,
}

impl MudClient {
    fn new() -> Self {
        let parser = setup_mud_parser();
        Self {
            parser,
            connected: false,
            supported_options: vec![
                op_option::GMCP,
                op_option::NAWS, 
                op_option::TTYPE,
            ],
        }
    }
    
    fn on_connect(&mut self) {
        self.connected = true;
        
        // Send initial negotiations
        for &option in &self.supported_options {
            if let Some(event) = self.parser._will(option) {
                // Send to server...
            }
        }
    }
    
    fn on_disconnect(&mut self) {
        self.connected = false;
        // Reset all negotiated states
        self.parser.options.reset_states();
    }
}
```

## Testing and Debugging

### Debug Output for Development
```rust
fn debug_telnet_events(events: Vec<TelnetEvents>) {
    for event in &events {
        match event {
            TelnetEvents::DataReceive(data) => {
                println!("RECV: {:?}", String::from_utf8_lossy(data));
            }
            TelnetEvents::DataSend(data) => {
                println!("SEND: {:?}", data.as_ref());
            }
            TelnetEvents::IAC(iac) => {
                println!("IAC: command={}", iac.command);
            }
            TelnetEvents::Negotiation(neg) => {
                println!("NEG: {}({}) {}({})", 
                    command_name(neg.command), neg.command,
                    option_name(neg.option), neg.option);
            }
            TelnetEvents::Subnegotiation(sub) => {
                println!("SUB: option={}, len={}", sub.option, sub.buffer.len());
            }
            TelnetEvents::DecompressImmediate(_) => {
                println!("DECOMP: MCCP data received");
            }
        }
    }
}

fn command_name(cmd: u8) -> &'static str {
    match cmd {
        op_command::WILL => "WILL",
        op_command::WONT => "WONT", 
        op_command::DO => "DO",
        op_command::DONT => "DONT",
        _ => "UNKNOWN",
    }
}

fn option_name(opt: u8) -> &'static str {
    match opt {
        op_option::ECHO => "ECHO",
        op_option::SGA => "SGA",
        op_option::NAWS => "NAWS",
        op_option::TTYPE => "TTYPE", 
        op_option::GMCP => "GMCP",
        _ => "UNKNOWN",
    }
}
```

This comprehensive example guide demonstrates the library's robust design and zero-copy efficiency. The parser handles all Telnet protocol complexity while providing a clean, safe API for MUD client development.