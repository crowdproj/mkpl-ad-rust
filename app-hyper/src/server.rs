use crate::routes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use tokio::net::TcpListener; // Добавлен импорт роутера

pub async fn run_server(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = addr.parse()?;
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(e) = http1::Builder::new()
                .serve_connection(io, service_fn(routes::handle_request))
                .await
            {
                eprintln!("Connection error: {e}");
            }
        });
    }
}
