use iron::IronResult;
use iron::request::Request;
use iron::response::Response;
use iron::status;

use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
}

pub trait WebHandler {
    fn endpoint(&self) -> &'static str;
    fn method(&self) -> Method;
    fn name(&self) -> &'static str;
    fn handle(&self) -> (Fn(&mut Request) -> IronResult<Response>);
}
