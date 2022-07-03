
// pub struct Session {
//     bucket: Bucket,
// }

// impl Session {
//     pub async fn start()
// }

pub mod bucket {
    use s3::{creds::Credentials, Bucket, Region};
   
    type S3DefaultResult = Result<(), Box<dyn std::error::Error>>;
    type S3BucketResult = Result<Bucket, Box<dyn std::error::Error>>;

    pub async fn connect() -> Result<Bucket, Box<dyn std::error::Error>>  {
        let region_name = "us-west-004".to_string();
        let endpoint = "https://s3.us-west-004.backblazeb2.com".to_string();
        let region = Region::Custom {
            region: region_name,
            endpoint,
        };

        let bucket_name = "Blazemaster";
        let credentials = Credentials::default().unwrap();

        let bucket = Bucket::new(bucket_name, region, credentials)?;

        Ok(bucket)
    }

    pub async fn upload_file(local_path: &str, remote_path: &str) -> S3DefaultResult {

        Ok(())
    }

    pub async fn download_file(bucket: &Bucket, remote_path: &str, local_path: &str) -> S3DefaultResult {
        let (data, code) = bucket.get_object(remote_path).await?;

        Ok(())
    }
    
    pub async fn list(bucket: &Bucket) -> S3DefaultResult {
        let results = bucket.list("".to_string(), None).await?;

        for result in results.iter() {
            for object in result.contents.iter() {
                println!("{:?}, ", object);
            } 
        }
        Ok(())
    } 
}

