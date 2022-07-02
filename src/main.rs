use s3::{creds::Credentials, Bucket, Region};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    connect_to_bucket().await?;
    Ok(())
}

async fn connect_to_bucket() -> Result<(), Box<dyn std::error::Error>>  {
    let region_name = "us-west-004".to_string();
    let endpoint = "https://s3.us-west-004.backblazeb2.com".to_string();
    let region = Region::Custom {
        region: region_name,
        endpoint,
    };

    let bucket_name = "Blazemaster";
    let credentials = Credentials::default().unwrap();

    let bucket = Bucket::new(bucket_name, region, credentials)?;

    let results = bucket.list("".to_string(), None).await?;

    for result in results.iter() {
        for object in result.contents.iter() {
            println!("{:?}, ", object);
        } 
    }

    let (data, code) = bucket.get_object("/monke.gif").await?;

    

    // println!("data = {:?}", data);

    Ok(())

}
