use borsalino::server;

fn main() {
    server::listen_at(String::from("127.0.0.1:8080"));
}
