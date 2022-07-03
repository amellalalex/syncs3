mod s3support;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bucket = s3support::connect().await?;
    s3support::list(&bucket).await?;

    Ok(())
}
