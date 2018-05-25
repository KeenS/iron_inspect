extern crate iron;
extern crate iron_inspect;

use iron::prelude::*;
use iron_inspect::Inspect;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct DummyError;
impl fmt::Display for DummyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for DummyError {
    fn cause(&self) -> Option<&Error> {
        None
    }

    fn description(&self) -> &str {
        "dummy"
    }
}

fn handler(req: &mut Request) -> IronResult<Response> {
    if req.url.path().iter().any(|s| s.contains("error")) {
        Err(IronError::new(DummyError, iron::status::BadRequest))
    } else {
        Ok(Response::with(iron::status::Ok))
    }
}

fn main() {
    let mut chain = Chain::new(handler);
    chain.link_before(Inspect::request(|req| println!("request {:?}", req)));
    chain.link_after(Inspect::new(|_req, res| match res {
        Ok(r) => println!("ok: {:?}", r),
        Err(e) => println!("err: {:?}", e),
    }));
    chain.link_after(Inspect::response(|_req, res| {
        println!("response: {:?}", res)
    }));

    chain.link_after(Inspect::error(|_req, err| println!("error: {:?}", err)));

    println!("access http://127.0.0.1:3000/ok and http://127.0.0.1:3000/error");
    Iron::new(chain).http("127.0.0.1:3000").unwrap();
}
