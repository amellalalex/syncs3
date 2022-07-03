mod s3support;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    s3support::connect_to_bucket().await?;
    Ok(())
}
