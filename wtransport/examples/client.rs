use std::time::Duration;
use wtransport::ClientConfig;
use wtransport::Endpoint;

#[tokio::main]
async fn main() {
    let config = ClientConfig::builder()
        .with_bind_default()
        .with_no_cert_validation()
        .build();

    let connection = Endpoint::client(config)
        .unwrap()
        .connect("https://[::1]:4433")
        .await
        .unwrap();

    let mut stream = connection.open_bi().await.unwrap().await.unwrap();
    stream.0.write_all(b"HELLO").await.unwrap();
    tokio::time::sleep(Duration::from_millis(1)).await;
    stream.0.write_all(b"HOWDY").await.map_err(|e| e.to_string()).expect("failed write");
    stream.0.finish().await.unwrap();
}
