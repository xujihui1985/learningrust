use crate::error::Error;

mod auth;
mod bucket;
mod error;
mod oss;

#[tokio::main]
async fn main() -> Result<(), Error> {
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

    let obj_name = "testfile1";
    oss.put_object("seanxu-testbucket2", obj_name, "helloworld").await?;
    let content = oss.get_object("seanxu-testbucket2", obj_name).await?;
    println!("res {}", String::from_utf8_lossy(content.as_slice()));

    Ok(())
}
