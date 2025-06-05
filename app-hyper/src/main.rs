#[macro_use]
mod macros;
mod routes;
mod server;

use server::run_server;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env("LOG_LEVEL").init();
    run_server("0.0.0.0:8080").await.expect("Server failed");
}
