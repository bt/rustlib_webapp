use handler::WebHandler;
use routing::WebRouter;

use iron::Handler;
use iron::prelude::*;

use mount::Mount;

use router::Router;

use staticfile::Static;

use std::collections::HashMap;

use std::path::Path;

use std::thread;
use std::thread::JoinHandle;

pub struct WebServer {
    interface: String,
    port: u32,
    child_thread: Option<JoinHandle<()>>,
    web_root: String,
    router: WebRouter,
}

impl WebServer {
    pub fn new(interface: &str, port: u32, web_root: &str, router: WebRouter) -> Self {
        WebServer {
            interface: interface.to_string(),
            port: port,
            child_thread: None,
            web_root: web_root.to_string(),
            router: router,
        }
    }

    pub fn bind(&mut self) {
        let bind_string = self.get_bind_string();

        let mount = self.init_mount(&self.web_root);

        let child_thread_handle = thread::spawn(move || {
            Iron::new(mount).http(&*bind_string).unwrap();
        });

        self.child_thread = Some(child_thread_handle);
    }

    fn get_bind_string(&self) -> String {
        format!("{}:{}", self.interface, self.port)
    }

    fn init_mount(&self, path: &str) -> Mount {
        let mut mount = Mount::new();
        mount.mount("/", Static::new(Path::new(path)))
            .mount("/api", self.router.to_iron_router());

        mount
    }
}
