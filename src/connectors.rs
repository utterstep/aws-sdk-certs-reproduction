use aws_config::{
    environment::{EnvironmentVariableCredentialsProvider, EnvironmentVariableRegionProvider},
    SdkConfig,
};
use aws_sdk_ec2::Client as EC2Client;
use aws_sdk_s3::Client as S3Client;
use aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder;

pub struct Connectors {
    pub ec2_client: EC2Client,
    pub s3_client: S3Client,
}

async fn aws_sdk_config() -> SdkConfig {
    // Rustls connector.
    let rustls_connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_only()
        .enable_http1()
        .enable_http2()
        .build();

    // Hyper client builder.
    let http_client = HyperClientBuilder::new().build(rustls_connector);

    aws_config::from_env()
        .http_client(http_client)
        .region(EnvironmentVariableRegionProvider::new())
        .credentials_provider(EnvironmentVariableCredentialsProvider::new())
        .load()
        .await
}

impl Connectors {
    pub async fn new() -> Result<Self, eyre::Report> {
        let aws_shared_config = aws_sdk_config().await;
        let s3_client = S3Client::new(&aws_shared_config);
        let ec2_client = EC2Client::new(&aws_shared_config);

        Ok(Self {
            ec2_client,
            s3_client,
        })
    }
}
