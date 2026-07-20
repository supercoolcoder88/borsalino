pub mod server {
    use std::net::TcpListener;

    pub fn start_server() {
        let listener = TcpListener::bind("127.0.0.1:8080").expect("failed to bind addr");

        for stream in listener.incoming() {
            match stream {
                Ok(_) => println!("heard"),
                Err(error) => eprintln!("connection error: {error}"),
            }
        }
    }
}
