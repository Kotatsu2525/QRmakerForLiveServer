use qrcode::QrCode;
use qrcode::render::unicode;
use std::net::IpAddr;
use std::net::TcpStream;
use std::net::UdpSocket;

fn main() {

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
    socket.connect("8.8.8.8:80").expect("Failed to connect");
    let local_address = socket.local_addr().unwrap().ip();

    match available_port(local_address) {
        true => {
            println!("Local IP address: {}", local_address);
            let url = format!("http://{}:5500/", local_address);
            println!("Access the server at: {}", url);
            let code = QrCode::new(url).unwrap();
            let string = code.render::<unicode::Dense1x2>().build();
            println!("{}", string);
        }
        _ => {
            println!("接続が失敗しました。確認してください。")
        }
    }
}

fn available_port(local_address: IpAddr) -> bool {
    let addr = format!("{}:5500", local_address);
    match TcpStream::connect(addr) {
        Ok(_) => true,
        Err(_) => false,
    }
}
