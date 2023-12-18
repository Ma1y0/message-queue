use message_queu::Message;

fn main() {
    let m = Message {
        input: "Hello".into(),
    };
    println!("Hello from client");
    println!("{:?}", m);
}
