use qrcode::QrCode;
use qrcode::render::unicode;
use std::env;
use std::net::IpAddr;
use std::net::TcpStream;
use std::net::UdpSocket;
use std::path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = match args.get(1) {
        Some(arg) => match arg.as_str() {
            "-h" | "--help" => {
                println!("使い方：");
                println!("  QRmakerForLS");
                println!("    ↳ 自動でポート番号を取得し、QRコードを作ります。");
                println!("  QRmakerForLS [ポート番号]");
                println!("    ↳ ポート番号を手動で設定することもできます。");

                None
            }
            port_str => match port_str.parse::<u16>() {
                Ok(port) => {
                    println!("指定されたポート番号: {}", port);

                    Some(port)
                }
                Err(_) => {
                    eprintln!("エラー: '{}' は有効なポート番号ではありません。", port_str);
                    println!("ヘルプは '-h' または '--help' を引数に渡してください。");

                    None
                }
            },
        },
        None => {
            readSettingFile()
        }
    };


    let path = match args.get(2) {
        Some(text) => {
            if text.chars().next() == "/" {
                text
            } else {
                println!("エラー: '{}' は有効なpathではありません。", text);
                println!("pathは '\' から始めてください。");
                panic!();
            }
        }
        None => {
            "/"
        }
    };

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
    socket.connect("8.8.8.8:80").expect("Failed to connect");
    let local_address = socket.local_addr().unwrap().ip();

    match available_port(local_address) {
        true => {
            println!("Local IP address: {}", local_address);
            //let url = format!("http://{}:5500/", local_address);
            let url = format!("http://{}:{}/", local_address, port);
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

fn readSettingFile() -> Option<u16> {
    Some(5500)
}
