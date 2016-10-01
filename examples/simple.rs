extern crate iron;
extern crate webapp;

use iron::Handler;
use iron::prelude::*;
use iron::status;

use std::collections::HashMap;

use webapp::Method;
use webapp::WebHandler;
use webapp::WebRouter;
use webapp::WebServer;

fn main() {
    let mut router = WebRouter::new();
    router.add_route(Box::new(Helloworld));

    let mut server = WebServer::new("localhost", 8080, "./examples/webroot", router);

    server.bind();

    while (true) {}
}

struct Helloworld;
impl WebHandler for Helloworld {
    fn endpoint(&self) -> &'static str {
        "/hello"
    }

    fn method(&self) -> Method {
        Method::Get
    }

    fn name(&self) -> &'static str {
        "Helloworld"
    }

    fn handler(&self) -> (Fn(&mut Request) -> IronResult<Response>) {
        |&mut Request| Ok(Response::with((status::Ok, "Hello world!".to_owned())))
    }
}
