use std::io::{self, ErrorKind, Read, Write};
use std::{
    net::TcpStream,
    sync::mpsc::{self, TryRecvError},
    thread,
    time::Duration,
};

const LOCAL: &str = "127.0.0.1:6000";
fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("Failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        loop {
            let mut buff = vec![0; 32];
            match client.read_exact(&mut buff) {
                Ok(_) => {}
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                Err(_) => {
                    println!("Connection with server was serverd");
                    break;
                }
            }
            match rx.try_recv() {
                Ok(msg) => {
                    let mut buff = msg.clone().into_bytes();
                    buff.resize(32, 0);
                    client.write_all(&buff).expect("writing to socket failed");
                    println!("message send {:?}", msg);
                }
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => break,
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
    println!("write a message");
    loop {
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {
            break;
        }
    }
    println!("bye bye!");
}
