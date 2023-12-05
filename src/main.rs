mod connectors;

#[tokio::main]
async fn main() {
    let connectors = connectors::Connectors::new().await.unwrap();

    println!("{:?}", connectors.ec2_client);
}
