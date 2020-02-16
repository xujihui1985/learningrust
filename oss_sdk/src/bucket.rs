use std::fmt::Write;

use chrono::prelude::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use reqwest::header::{HeaderMap, DATE};

use crate::auth::Auth;
use crate::error::Error;

use super::oss::OSS;

#[derive(Debug)]
pub struct Bucket {
    pub name: String,
    pub creation_date: String,
}

#[derive(Debug)]
pub struct Object {
    pub key: String,
    pub size: String,
    pub last_modified: String,
}

impl Object {
    pub fn new(key: &str, last_modified: &str, size: &str) -> Self {
        Object {
            key: key.to_owned(),
            size: size.to_owned(),
            last_modified: last_modified.to_owned(),
        }
    }
}

impl OSS {
    pub async fn get_object(&self, bucket_name: &str, obj_name: &str) -> Result<Vec<u8>, Error> {
        let client = reqwest::Client::new();
        let url = format!(
            "http://{}.{}/{}",
            bucket_name,
            self.get_endpoint(),
            obj_name
        );
        let now = Utc::now();
        let date = now.format("%a, %d %b %Y %T GMT").to_string();
        let mut headers = HeaderMap::new();
        headers.insert(DATE, date.parse()?);
        let auth = self.oss_sign("GET", bucket_name, obj_name, "", &headers);
        headers.insert("Authorization", auth.parse()?);
        let res = client.get(&url).headers(headers).send().await?;
        if res.status().is_success() {
            let body = res.text().await?;
            Ok(Vec::from(body))
        } else {
            println!("error {}", res.status());
            Err(Error::App)
        }
    }

    pub async fn put_object(
        &self,
        bucket_name: &str,
        object_name: &str,
        content: impl AsRef<[u8]>,
    ) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let url = format!(
            "http://{}.{}/{}",
            bucket_name,
            self.get_endpoint(),
            object_name
        );
        let now = Utc::now();
        let date = now.format("%a, %d %b %Y %T GMT").to_string();
        let mut headers = HeaderMap::new();
        headers.insert(DATE, date.parse()?);
        let auth = self.oss_sign("PUT", bucket_name, object_name, "", &headers);
        headers.insert("Authorization", auth.parse()?);
        let res = client.put(&url).headers(headers).body(Vec::from(content.as_ref())).send().await?;
        if res.status().is_success() {
            println!("success");
        } else {
            println!("error {}", res.status());
        }
        Ok(())
    }

    pub async fn put_bucket(&self, bucket_name: &str) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let url = format!("http://{}.{}", bucket_name, self.get_endpoint());

        let now = Utc::now();
        let date = now.format("%a, %d %b %Y %T GMT").to_string();
        let acl = "public-read-write";
        let mut headers = HeaderMap::new();
        headers.insert(DATE, date.parse()?);
        headers.insert("x-oss-acl", acl.parse()?);
        let auth = self.oss_sign("PUT", bucket_name, "", "", &headers);
        headers.insert("Authorization", auth.parse()?);
        let mut storage_class = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        write!(&mut storage_class, "<CreateBucketConfiguration>").unwrap();
        writeln!(
            &mut storage_class,
            "<StorageClass>{}</StorageClass>",
            "Standard"
        )
        .unwrap();
        writeln!(&mut storage_class, "</CreateBucketConfiguration>").unwrap();
        let b = Vec::from(storage_class.as_bytes());
        let res = client.put(&url).headers(headers).body(b).send().await?;
        if res.status().is_success() {
            println!("success");
        } else {
            println!("error {}", res.status());
        }

        Ok(())
    }

    pub async fn list_buckets(&self) -> Result<Vec<Bucket>, Error> {
        let client = reqwest::Client::new();
        let url = format!("http://{}", self.get_endpoint());
        let mut headers = HeaderMap::new();
        let now = Utc::now();
        let date = now.format("%a, %d %b %Y %T GMT").to_string();
        headers.insert(DATE, date.parse()?);
        let auth = self.oss_sign("GET", "", "", "", &headers);
        headers.insert("Authorization", auth.parse()?);
        let res = client.get(&url).headers(headers).send().await?;
        let mut buckets: Vec<Bucket> = Vec::new();
        if res.status().is_success() {
            let body = res.text().await?;
            let mut reader = Reader::from_str(body.as_str());
            reader.trim_text(true);
            let mut buf = Vec::<u8>::new();
            let mut name: Option<String> = None;
            let mut creation_date: Option<String> = None;
            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(e)) => match e.name() {
                        b"Name" => {
                            name = Some(reader.read_text(e.name(), &mut Vec::new()).unwrap());
                        }
                        b"CreationDate" => {
                            creation_date =
                                Some(reader.read_text(e.name(), &mut Vec::new()).unwrap());
                        }
                        _ => (),
                    },
                    Ok(Event::End(e)) => match e.name() {
                        b"Bucket" => {
                            let n = name.clone().unwrap_or_default();
                            let l = creation_date.clone().unwrap_or_default();
                            let b = Bucket {
                                name: n,
                                creation_date: l,
                            };
                            buckets.push(b);
                        }
                        _ => (),
                    },
                    Ok(Event::Eof) => break,
                    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                    _ => (),
                }
            }
            Ok(buckets)
        } else {
            Err(Error::App)
        }
    }

    pub async fn list_objects(&self, bucket_name: &str) -> Result<Vec<Object>, Error> {
        let client = reqwest::Client::new();
        let url = format!("http://{}.{}", bucket_name, self.get_endpoint());
        let mut headers = HeaderMap::new();
        let now = Utc::now();
        let date = now.format("%a, %d %b %Y %T GMT").to_string();
        headers.insert(DATE, date.parse()?);
        let auth = self.oss_sign("GET", bucket_name, "", "", &headers);
        headers.insert("Authorization", auth.parse()?);
        let res = client.get(&url).headers(headers).send().await?;
        if res.status().is_success() {
            let body = res.text().await?;
            let mut reader = Reader::from_str(body.as_str());
            reader.trim_text(true);
            let mut buf = Vec::<u8>::new();
            let mut name: Option<String> = None;
            let mut size: Option<String> = None;
            let mut last_modified: Option<String> = None;
            let mut objects: Vec<Object> = Vec::new();

            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(ref e)) => match e.name() {
                        b"Key" => {
                            name = Some(reader.read_text(e.name(), &mut Vec::new()).unwrap());
                        }
                        b"LastModified" => {
                            last_modified =
                                Some(reader.read_text(e.name(), &mut Vec::new()).unwrap());
                        }
                        b"Size" => {
                            size = Some(reader.read_text(e.name(), &mut Vec::new()).unwrap());
                        }
                        _ => (),
                    },
                    Ok(Event::End(ref e)) => match e.name() {
                        b"Contents" => {
                            let n = name.clone().unwrap_or_default();
                            let l = last_modified.clone().unwrap_or_default();
                            let s = size.clone().unwrap_or_default();
                            let b = Object::new(&n, &l, &s);
                            objects.push(b);
                        }
                        _ => (),
                    },
                    Ok(Event::Eof) => break,
                    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                    _ => (),
                }
            }
            Ok(objects)
        } else {
            Err(Error::App)
        }
    }
}
