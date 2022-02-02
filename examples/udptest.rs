use serde_json::Value;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:50222").expect("bind failure");

    let mut buf = [0; 400];
    for _ in 1..=10 {
        let (amt, src) = socket.recv_from(&mut buf).expect("no data");
        println!(
            "recv: {} {} {:?}",
            amt,
            src,
            std::str::from_utf8(&buf[..amt])
        );
        println!("before match");
        match std::str::from_utf8(&buf[..amt]) {
            Ok(s) => {
                let v: Value = serde_json::from_str(s)?;
                println!("decoded json {:?}", &v);
            }
            Err(e) => println!("error {:?}", e),
        }
        println!("after match");
    }
    Ok(())
}
