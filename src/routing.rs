use iron::Handler;

use handler::WebHandler;

use router::Router;

pub struct WebRouter {
    routes: Vec<Box<WebHandler>>,
}

impl WebRouter {
    pub fn new() -> Self {
        WebRouter { routes: Vec::new() }
    }

    pub fn add_route(&mut self, handler: Box<WebHandler>) {
        self.routes.push(handler);
    }

    pub fn to_iron_router(&self) -> Router {
        let mut router = Router::new();

        for handler in &self.routes {
            let () = handler.handle();
            // router.get(handler.endpoint(), handler.handle(), handler.name());
        }

        router
    }
}
