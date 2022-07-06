use http::method::Method;
use server::Server;
use http::request::Request;

mod http;
mod server;

fn main() {
    /* 
    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;
    */
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

