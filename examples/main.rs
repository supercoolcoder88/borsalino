use borsalino::http;
use borsalino::http_method::HttpMethod;

fn main() {
    // Listening on a addr
    let mut server = http::Router::default();

    // Routing
    server.add_route(HttpMethod::Get, String::from("/health"), health_check);

    server.listen_at(String::from("127.0.0.1:8080"));
}

fn health_check(w: &mut http::ResponseWriter) {
    w.write("HTTP/1.1 200 OK\r\nContent-Length: 7\r\n\r\nTest123")
}
