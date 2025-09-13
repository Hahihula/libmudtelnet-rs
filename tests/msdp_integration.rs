use bytes::Bytes;
use libmudtelnet_rs::telnet::{msdp, op_command as cmd, op_option as opt};
use libmudtelnet_rs::{
  events::{TelnetEvents, TelnetSubnegotiation},
  Parser,
};

#[test]
fn parser_handles_msdp_subnegotiation() {
  let mut parser = Parser::new();
  parser.options.support_local(opt::MSDP);
  parser._will(opt::MSDP);

  // Test MSDP subneg with typical VAR/VAL structure: HP = 100
  let payload = Bytes::from(vec![msdp::VAR, b'H', b'P', msdp::VAL, b'1', b'0', b'0']);
  let subneg_bytes = TelnetSubnegotiation::new(opt::MSDP, payload.clone()).to_bytes();

  let events = parser.receive(&subneg_bytes);
  assert_eq!(events.len(), 1);

  match &events[0] {
    TelnetEvents::Subnegotiation(sub) => {
      assert_eq!(sub.option, opt::MSDP);
      // Verify parser preserved the MSDP structure exactly
      assert_eq!(sub.buffer, payload);
    }
    _ => panic!("expected Subnegotiation event"),
  }
}

#[test]
fn parser_handles_empty_msdp_subnegotiation() {
  let mut parser = Parser::new();
  parser.options.support_local(opt::MSDP);
  parser._will(opt::MSDP);

  // Empty MSDP subneg (edge case)
  let empty_payload = Bytes::new();
  let subneg_bytes = TelnetSubnegotiation::new(opt::MSDP, empty_payload).to_bytes();
  let events = parser.receive(&subneg_bytes);

  assert_eq!(events.len(), 1);
  match &events[0] {
    TelnetEvents::Subnegotiation(sub) => {
      assert_eq!(sub.option, opt::MSDP);
      assert!(sub.buffer.is_empty());
    }
    _ => panic!("expected empty Subnegotiation"),
  }
}

#[test]
fn parser_handles_msdp_table_structure() {
  let mut parser = Parser::new();
  parser.options.support_local(opt::MSDP);
  parser._will(opt::MSDP);

  // Test MSDP table structure: ROOM = { VNUM: "1234", NAME: "Test Room" }
  let payload = Bytes::from(vec![
    msdp::VAR,
    b'R',
    b'O',
    b'O',
    b'M',
    msdp::VAL,
    msdp::TABLE_OPEN,
    msdp::VAR,
    b'V',
    b'N',
    b'U',
    b'M',
    msdp::VAL,
    b'1',
    b'2',
    b'3',
    b'4',
    msdp::VAR,
    b'N',
    b'A',
    b'M',
    b'E',
    msdp::VAL,
    b'T',
    b'e',
    b's',
    b't',
    b' ',
    b'R',
    b'o',
    b'o',
    b'm',
    msdp::TABLE_CLOSE,
  ]);
  let subneg_bytes = TelnetSubnegotiation::new(opt::MSDP, payload.clone()).to_bytes();

  let events = parser.receive(&subneg_bytes);
  assert_eq!(events.len(), 1);

  match &events[0] {
    TelnetEvents::Subnegotiation(sub) => {
      assert_eq!(sub.option, opt::MSDP);
      // Verify parser preserved all MSDP tags and structure
      assert_eq!(sub.buffer, payload);
    }
    _ => panic!("expected Subnegotiation event"),
  }
}

#[test]
fn parser_handles_msdp_array_structure() {
  let mut parser = Parser::new();
  parser.options.support_local(opt::MSDP);
  parser._will(opt::MSDP);

  // Test MSDP array: EXITS = ["north", "south", "east"]
  let payload = Bytes::from(vec![
    msdp::VAR,
    b'E',
    b'X',
    b'I',
    b'T',
    b'S',
    msdp::VAL,
    msdp::ARRAY_OPEN,
    msdp::VAL,
    b'n',
    b'o',
    b'r',
    b't',
    b'h',
    msdp::VAL,
    b's',
    b'o',
    b'u',
    b't',
    b'h',
    msdp::VAL,
    b'e',
    b'a',
    b's',
    b't',
    msdp::ARRAY_CLOSE,
  ]);
  let subneg_bytes = TelnetSubnegotiation::new(opt::MSDP, payload.clone()).to_bytes();

  let events = parser.receive(&subneg_bytes);
  assert_eq!(events.len(), 1);

  match &events[0] {
    TelnetEvents::Subnegotiation(sub) => {
      assert_eq!(sub.option, opt::MSDP);
      assert_eq!(sub.buffer, payload);
    }
    _ => panic!("expected Subnegotiation event"),
  }
}

#[test]
fn parser_negotiates_msdp_option() {
  let mut parser = Parser::new();
  parser.options.support_local(opt::MSDP);

  // Should generate WILL MSDP
  if let Some(event) = parser._will(opt::MSDP) {
    match event {
      TelnetEvents::DataSend(data) => {
        assert_eq!(&data[..], &[cmd::IAC, cmd::WILL, opt::MSDP]);
      }
      _ => panic!("expected DataSend for WILL MSDP"),
    }
  } else {
    panic!("expected Some event for WILL MSDP");
  }
}

#[test]
fn parser_negotiates_other_new_options() {
  let mut parser = Parser::new();

  // Test CHARSET option
  parser.options.support_local(opt::CHARSET);
  if let Some(event) = parser._will(opt::CHARSET) {
    match event {
      TelnetEvents::DataSend(data) => {
        assert_eq!(&data[..], &[cmd::IAC, cmd::WILL, opt::CHARSET]);
      }
      _ => panic!("expected DataSend for WILL CHARSET"),
    }
  } else {
    panic!("expected Some event for WILL CHARSET");
  }

  // Test MXP option
  parser.options.support_local(opt::MXP);
  if let Some(event) = parser._will(opt::MXP) {
    match event {
      TelnetEvents::DataSend(data) => {
        assert_eq!(&data[..], &[cmd::IAC, cmd::WILL, opt::MXP]);
      }
      _ => panic!("expected DataSend for WILL MXP"),
    }
  } else {
    panic!("expected Some event for WILL MXP");
  }

  // Test ATCP option
  parser.options.support_local(opt::ATCP);
  if let Some(event) = parser._will(opt::ATCP) {
    match event {
      TelnetEvents::DataSend(data) => {
        assert_eq!(&data[..], &[cmd::IAC, cmd::WILL, opt::ATCP]);
      }
      _ => panic!("expected DataSend for WILL ATCP"),
    }
  } else {
    panic!("expected Some event for WILL ATCP");
  }
}
