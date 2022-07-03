mod s3support;
use s3support::bucket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bucket = bucket::connect().await?;
    bucket::list(&bucket).await?;

    Ok(())
}
