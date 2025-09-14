use libmudtelnet_rs::telnet::op_option;
use libmudtelnet_rs::{events::TelnetEvents, Parser};

fn main() {
  // Minimal demo: feed a few bytes and print parsed events
  let mut parser = Parser::new();

  // Pretend this came from the network: "hi", doubled IAC, CRLF
  let events = parser.receive(b"hi \xFF\xFF\r\n");
  for ev in events {
    match ev {
      TelnetEvents::DataReceive(buf) => {
        println!("DATA  : {:?}", String::from_utf8_lossy(&buf));
      }
      TelnetEvents::Negotiation(n) => {
        println!("NEG   : cmd={} opt={}", n.command, n.option);
      }
      TelnetEvents::Subnegotiation(s) => {
        println!("SUB   : opt={} len={}", s.option, s.buffer.len());
      }
      TelnetEvents::IAC(i) => {
        println!("IAC   : cmd={}", i.command);
      }
      TelnetEvents::DecompressImmediate(b) => {
        println!("MCCP  : boundary len={}", b.len());
      }
      TelnetEvents::DataSend(buf) => {
        // Bytes that should be written to the socket as-is
        println!("SEND  : {} bytes", buf.len());
      }
      _ => {}
    }
  }

  // Example: send a GMCP payload (application would write buf to the socket)
  if let Some(TelnetEvents::DataSend(buf)) =
    parser.subnegotiation_text(op_option::GMCP, "{\"Core.Ping\":1}")
  {
    println!("SEND  : {} bytes (GMCP)", buf.len());
  }
}
