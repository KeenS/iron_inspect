extern crate iron;
use iron::prelude::*;
use iron::middleware::AfterMiddleware;

use std::marker::PhantomData;

pub enum InspectResult {}
pub enum InspectResponse {}
pub enum InspectError {}

pub struct Inspect<F, What> {
    f: F,
    _phantom: PhantomData<What>,
}

impl<F> Inspect<F, ()> {
    pub fn new(f: F) -> Inspect<F, InspectResult>
    where
        F: Fn(&Request, Result<&Response, &IronError>),
    {
        Inspect {
            f,
            _phantom: PhantomData,
        }
    }

    pub fn response(f: F) -> Inspect<F, InspectResponse>
    where
        F: Fn(&Request, &Response),
    {
        Inspect {
            f,
            _phantom: PhantomData,
        }
    }

    pub fn error(f: F) -> Inspect<F, InspectError>
    where
        F: Fn(&Request, &IronError),
    {
        Inspect {
            f,
            _phantom: PhantomData,
        }
    }
}



impl<F> AfterMiddleware for Inspect<F, InspectResult>
where
    F: Send + Sync + 'static,
    F: Fn(&Request, Result<&Response, &IronError>),
{
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        (self.f)(req, Ok(&res));
        Ok(res)
    }

    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        (self.f)(req, Err(&err));
        Err(err)
    }
}

impl<F> AfterMiddleware for Inspect<F, InspectResponse>
where
    F: Send + Sync + 'static,
    F: Fn(&Request, &Response),
{
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        (self.f)(req, &res);
        Ok(res)
    }
}

impl<F> AfterMiddleware for Inspect<F, InspectError>
where
    F: Send + Sync + 'static,
    F: Fn(&Request, &IronError),
{
    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        (self.f)(req, &err);
        Err(err)
    }
}
