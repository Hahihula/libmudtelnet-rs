// A minimal async Tokio integration example.
// This file intentionally avoids pulling `tokio` as a dependency of the crate
// (which would require network). Copy the snippet below into your app where
// you already depend on `tokio`:
//
// ```rust,ignore
// use libmudtelnet_rs::{events::TelnetEvents, Parser};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::TcpStream;
//
// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let mut stream = TcpStream::connect("mud.example.org:4000").await?;
//     let (mut r, mut w) = stream.split();
//     let mut parser = Parser::new();
//     let mut buf = vec![0u8; 4096];
//
//     loop {
//         let n = r.read(&mut buf).await?;
//         if n == 0 { break; }
//         for ev in parser.receive(&buf[..n]) {
//             match ev {
//                 TelnetEvents::DataReceive(data) => print!("{}", String::from_utf8_lossy(&data)),
//                 TelnetEvents::DataSend(data) => w.write_all(&data).await?,
//                 TelnetEvents::DecompressImmediate(block) => {
//                     // decompress then feed back
//                     let more = parser.receive(&block);
//                     for ev2 in more { if let TelnetEvents::DataReceive(d)=ev2 { print!("{}", String::from_utf8_lossy(&d)); }}
//                 }
//                 _ => {}
//             }
//         }
//     }
//     Ok(())
// }
// ```
fn main() {
  eprintln!("Tokio example is a template. Copy it into your app that depends on tokio.");
}
