#[macro_use]
extern crate hyper;
extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;

pub use self::handler::Method;
pub use self::handler::WebHandler;
pub use self::routing::WebRouter;
pub use self::server::WebServer;

mod handler;
mod routing;
mod server;
