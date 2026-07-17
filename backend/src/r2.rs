use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;

use crate::config::Config;
use crate::errors::AppError;

#[derive(Clone)]
pub struct R2Client {
    client: Client,
    bucket: String,
    public_url: String,
}

impl R2Client {
    pub fn new(cfg: &Config) -> Self {
        let endpoint = format!("https://{}.r2.cloudflarestorage.com", cfg.r2_account_id);

        let credentials = Credentials::new(
            &cfg.r2_access_key_id,
            &cfg.r2_secret_access_key,
            None,
            None,
            "r2",
        );

        let s3_config = aws_sdk_s3::Config::builder()
            .region(Region::new("auto"))
            .endpoint_url(endpoint)
            .credentials_provider(credentials)
            .behavior_version_latest()
            .build();

        R2Client {
            client: Client::from_conf(s3_config),
            bucket: cfg.r2_bucket_name.clone(),
            public_url: cfg.r2_public_url.clone(),
        }
    }

    pub async fn upload_image(&self, key: &str, bytes: Vec<u8>) -> Result<String, AppError> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(ByteStream::from(bytes))
            .content_type("image/png")
            .send()
            .await
            .map_err(|e| AppError::ImageGeneration(format!("R2 upload failed: {e}")))?;

        Ok(format!("{}/{}", self.public_url.trim_end_matches('/'), key))
    }
}