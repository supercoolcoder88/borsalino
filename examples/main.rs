use borsalino::http;
use borsalino::http_method::HttpMethod;

fn main() {
    let mut server = http::Router::default();

    // Routing
    server.add_route(HttpMethod::Get, String::from("/health"), health_check);
    server.add_route(HttpMethod::Get, String::from("/error"), error);
    server.add_route(HttpMethod::Get, String::from("/error2"), custom_error);

    server.listen_at(String::from("127.0.0.1:8080"));
}

fn health_check(w: &mut http::ResponseWriter) {
    w.ok("testing123");
}

fn error(w: &mut http::ResponseWriter) {
    w.error(
        http::HttpStatus::InternalServerError,
        http::HttpStatus::InternalServerError.default_msg(),
    );
}

fn custom_error(w: &mut http::ResponseWriter) {
    w.error(http::HttpStatus::BadRequest, "bad inputs");
}
