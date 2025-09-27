use clap::Parser;
use qrcode::QrCode;
use qrcode::render::unicode;
use std::io::{self, Write};
use std::net::IpAddr;
use std::net::TcpStream;
use std::net::UdpSocket;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 5500)]
    port: u16,

    #[arg(long, default_value = "/")]
    path: String,
    ///nomal QR maker. input text and make a QR.
    #[arg(long)]
    qr: bool,
}

fn main() {
    let args = Args::parse();

    if args.qr == false {
        qr_for_port(args);
    } else {
        nomal_qr_maker();
    }
}

fn qr_for_port(args: Args) {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
    socket.connect("8.8.8.8:80").expect("Failed to connect");
    let local_address = socket.local_addr().unwrap().ip();

    match available_port(local_address) {
        true => {
            println!("Local IP address: {}", local_address);
            let url = format!("http://{}:{}{}", local_address, args.port, args.path);
            println!("Access the server at: {}", url);
            let code = QrCode::new(url).unwrap();
            let string = code.render::<unicode::Dense1x2>().build();
            println!("{}", string);
        }
        _ => {
            println!("接続が失敗しました。確認してください。")
        }
    }
    println!("任意のキーを押して終了できます。");
    let mut finish = String::new();
    io::stdin().read_line(&mut finish).unwrap();
}

fn nomal_qr_maker() {
    loop {
        println!("QRコードにするURLなどを入力してください。(^C で終了)");
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("読み込みに失敗しました。");
        let input = input.trim();
        let code = QrCode::new(input).unwrap();
        let string = code.render::<unicode::Dense1x2>().build();
        println!("{}", string);
    }
}

fn available_port(local_address: IpAddr) -> bool {
    let addr = format!("{}:5500", local_address);
    match TcpStream::connect(addr) {
        Ok(_) => true,
        Err(_) => false,
    }
}
