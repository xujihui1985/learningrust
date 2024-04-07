use std::{net::SocketAddr, sync::Arc};
use tokio::{sync::mpsc, task::JoinHandle};
use tonic::transport::{self, Server};
use tonic_health::{pb::HealthCheckRequest, ServingStatus};

#[tokio::main]
async fn main() {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_service_status("", ServingStatus::Serving)
        .await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(health_service)
            .serve(addr)
            .await
            .expect("failed to start server");
    });
    handle.await.unwrap();
}
