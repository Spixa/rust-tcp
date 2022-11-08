enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

impl IpAddr {
    fn to_string(&self) -> String {
        match self {
            IpAddr::V4(a, b, c, d) => return format!("{a}.{b}.{c}.{d}"),
            IpAddr::V6(string) => return string.to_string()
        }
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct Server {
    ip: IpAddr,
}

impl Server {
    fn send(&self, msg: &Message) {
        print!("Sending to {} => ", self.ip.to_string());
        match msg {
            Message::Quit => {
                println!("Quit");
            }
            Message::Move { x, y } => {
                println!("Moved to: {x} {y}");
            }
            Message::Write(string) => {
                println!("Sent: {string}");
            }
            Message::ChangeColor(x, y, z) => {
                println!("Changed color to: {:?}", (x,y,z));
            }
        }
    }
}

impl Message {
    fn call(&self, server: &Server) {
        server.send(self);
    }
}


fn main() {
    let sv = Server {ip: IpAddr::V4(127,0,0,1) };
    let sv2 = Server {ip: IpAddr::V6("::1".into()) };

    let m = Message::Write(String::from("hello"));
    m.call(&sv);

    let color = Message::ChangeColor(255, 255, 255);
    color.call(&sv);

    let move_msg = Message::Move{x: 16, y:32};
    move_msg.call(&sv);

    let quit_server = Message::Quit;
    quit_server.call(&sv);

    m.call(&sv2);
    color.call(&sv2);
    move_msg.call(&sv2);
    quit_server.call(&sv2);
}
