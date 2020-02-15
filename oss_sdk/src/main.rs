use crate::error::Error;

mod auth;
mod bucket;
mod error;
mod oss;

#[tokio::main]
async fn main() -> Result<(), Error>{
    let endpoint = "";
    let access_key_id = "";
    let access_key_secret = "";
    let oss = oss::OSS::new(endpoint, access_key_id, access_key_secret);
    println!("call list bucket");
    let buckets = oss.list_buckets().await?;
    for b in buckets.iter() {
        println!("buckets {:?}", b);
    }
    let res = oss.list_objects("seanxu").await?;
    println!("total {}", res.len());
    for b in res.into_iter() {
        println!("objects {:?}", b);
    }

    oss.put_bucket("seanxu-testbucket2").await?;

    Ok(())
}
