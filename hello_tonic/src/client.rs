mod helloworld;

use std::net::SocketAddr;

use helloworld::greeter_client::GreeterClient;
use helloworld::HelloRequest;
use tonic::transport::{Channel, Server};
use tonic_health::{pb::{
    health_client::HealthClient as HealthGRPCClient, HealthCheckRequest, HealthCheckResponse,
}, ServingStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

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

    let channel = Channel::from_static("http://127.0.0.1:8081")
        .connect()
        .await?;

    let mut client = HealthGRPCClient::new(channel);
    let request = tonic::Request::new(HealthCheckRequest { service: "".to_string() });
    let res = client.check(request).await?;
    let resp = res.into_inner();
    println!("RESPONSE={:?}", resp);
    // let mut client = GreeterClient::connect("http://127.0.0.1:8081").await?;

    // let request = tonic::Request::new(HelloRequest {
    //     name: "Tonic".into(),
    // });

    // let response = client.say_hello(request).await?;

    // println!("RESPONSE={:?}", response);

    Ok(())
}
