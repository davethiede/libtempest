use serde_json::Value;
use std::net::UdpSocket;

use clap::Parser;

#[derive(Debug, clap::ArgEnum, Clone, Parser)]
enum Mode {
    Struct,
    Raw,
    Parsed,
}

/// Read tempest json weatherflow packets from a network interface
/// and display the resulting data. The default addr value will
/// read packets broadcast on the local subnet on port 50222.
#[derive(Debug, Parser)]
struct Arg {
    /// Exit after processing `count` packets.
    #[clap(long, short, value_name = "#")]
    count: Option<usize>,

    /// Packet buffer size.
    ///
    /// Buffer must be large enough to hold entire tempest UDP packet.
    #[clap(long, short, value_name = "#", default_value_t = 400)]
    bufsize: usize,

    /// Listen addr:port
    #[clap(long, short, default_value = "0.0.0.0:50222")]
    addr: String,

    /// Display mode.
    ///
    /// struct: Parse into `Tempest`.{n}
    /// parsed: Parse into generic `serde_json::Value`.{n}
    /// raw   : Display the text obtained from the packet.
    #[clap(arg_enum, short, long, default_value_t=Mode::Struct)]
    mode: Mode,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args = Arg::parse();
    let mut buf = vec![0_u8; args.bufsize];

    let socket = UdpSocket::bind(args.addr).expect("bind failure");
    let mut n: usize = 0;

    loop {
        let (amt, src) = socket.recv_from(&mut buf).expect("no data");
        match &args.mode {
            Mode::Raw => {
                println!("recv: {amt} {src} {:?}", std::str::from_utf8(&buf[..amt]));
            }
            Mode::Parsed => {
                let s = std::str::from_utf8(&buf[..amt])?;
                let v: Value = serde_json::from_str(s)?;
                println!("decoded json {:?}", &v);
            }
            Mode::Struct => {
                let v: tempest::Tempest = serde_json::from_slice(&buf[..amt])?;
                println!("tempest::Tempest = {v:?}");
            }
        }

        // if count specified, then only process that many packets and stop.
        if let Some(m) = args.count {
            n = n + 1;
            if n >= m {
                break;
            }
        }
    }
    Ok(())
}
