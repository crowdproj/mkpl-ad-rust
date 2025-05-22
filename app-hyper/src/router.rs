use std::error::Error;
use std::task::{Context, Poll};

use api_v1::server::MakeService;
use futures::future::BoxFuture;
// use api_v1::context::MakeAddContext;
// use api_v1::server::MakeService;
use futures_util::future;
use hyper::service::Service;
use hyper::{Body, Request, Response, Server};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::{EmptyContext, Has, XSpanIdString};

// use crate::server::Server;

const ROOT: &str = "/";

// #[derive(Debug)]
// pub struct RootService;

// impl Service<Request<Body>> for RootService {
//     type Response = Response<Body>;
//     type Error = hyper::Error;
//     type Future = future::Ready<Result<Self::Response, Self::Error>>;

//     fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         Ok(()).into()
//     }

//     fn call(&mut self, req: Request<Body>) -> Self::Future {
//         let rsp = Response::builder();

//         let uri = req.uri();
//         if uri.path() != ROOT {
//             let body = Body::from(Vec::new());
//             let rsp = rsp.status(404).body(body).unwrap();
//             return future::ok(rsp);
//         }

//         let body = Body::from(Vec::from(&b"heyo!"[..]));
//         let rsp = rsp.status(200).body(body).unwrap();
//         future::ok(rsp)
//     }
// }

pub struct RouterService {
    pub v1_service:
        api_v1::context::MakeAddContext<MakeAllowAllAuthenticator<MakeService<Server<_>, _>, _>, _>,
}

impl<C> RouterService<C> {
    pub fn new(server: crate::server::Server<C>) -> Self {
        let service = api_v1::server::MakeService::new(server);
        let service = MakeAllowAllAuthenticator::new(service, "cosmo");
        let mut service = api_v1::server::context::MakeAddContext::<_, EmptyContext>::new(service);
        RouterService {
            v1_service: service,
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for RouterService
where
    T: Clone + Send + Sync + 'static,
{
    type Response = Response<Body>;
    type Error = dyn Error;
    type Future = BoxFuture<'static, Result<Response<Body>, dyn Error>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future {
        async fn run<T, C>(
            mut api_impl: T,
            req: (Request<Body>, C),
        ) -> Result<Response<Body>, dyn Error>
        where
            T: Clone + Send + 'static,
        {
            let (request, context) = req;
            let (parts, body) = request.into_parts();
            let (method, uri, headers) = (parts.method, parts.uri, parts.headers);

            future::ok(self.v1_service)
        }
    }
}
