use failure::Fallible;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::ClientBuilder;
use reqwest::Method;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Read;
use std::time::Duration;
use url::Url;

type Headers = HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct Config {
    pub user_agent: String,
    pub resume: bool,
    pub headers: Headers,
    pub fname: String,
    pub timeout: u64,
    pub concurrent: bool,
    pub max_retries: i32,
    pub num_workers: usize,
    pub bytes_on_disk: Option<u64>,
    pub chunk_offsets: Option<Vec<(u64, u64)>>,
    pub chunk_size: u64,
}

#[allow(unused_variables)]
pub trait EventsHandler {
    fn on_headers(&mut self, header: &HeaderMap) {}
    fn on_server_supports_resume(&mut self) {}
    fn on_finish(&mut self) {}
    fn on_content(&mut self, contents: &[u8]) -> Fallible<()> {
        println!("{}", String::from_utf8_lossy(contents));
        Ok(())
    }
}

pub struct HttpDownload {
    url: Url,
    hooks: Vec<RefCell<Box<dyn EventsHandler>>>,
    conf: Config,
    retries: i32,
    http_client: reqwest::Client,
}

impl HttpDownload {
    pub fn new(url: Url, conf: Config) -> Fallible<HttpDownload> {
        let t = Duration::new(conf.timeout, 0);
        let client = ClientBuilder::new().timeout(t).build()?;
        Ok(HttpDownload {
            url,
            hooks: Vec::new(),
            conf,
            retries: 0,
            http_client: client,
        })
    }

    pub fn download(&mut self) -> Fallible<()> {
        let resp = self.http_client.head(self.url.as_str()).send()?;
        let support_bytes = match resp.headers().get("Accept-Ranges") {
            Some(val) => val == "bytes",
            None => false,
        };

        if support_bytes && self.conf.headers.get("Range").is_some() {
            if self.conf.concurrent {
                self.conf.headers.remove("Range");
            }
            for hook in &self.hooks {
                hook.borrow_mut().on_server_supports_resume();
            }
        }

        let req = self
            .http_client
            .request(Method::GET, self.url.as_str())
            .build()?;
        if resp.status().is_success() {
            for hk in &self.hooks {
                hk.borrow_mut().on_headers(resp.headers());
            }

            if support_bytes && self.conf.concurrent {
                self.concurrent_download(req, resp.headers().get("Content-Length"))?;
            } else {
                self.singlethread_download(req)?;
            }
        } else {
            println!("{}, {}", self.url.as_str(), resp.status().as_str());
        }

        Ok(())
    }

    pub fn events_hook<E>(&mut self, hk: E) -> &mut HttpDownload
    where
        E: EventsHandler + 'static,
    {
        self.hooks.push(RefCell::new(Box::new(hk)));
        self
    }

    fn concurrent_download(
        &mut self,
        req: reqwest::Request,
        ct_val: Option<&HeaderValue>,
    ) -> Fallible<()> {
        Ok(())
    }

    fn singlethread_download(&mut self, req: reqwest::Request) -> Fallible<()> {
        let mut resp = self.http_client.execute(req)?;
        let ct_len = if let Some(val) = resp.headers().get("Content-Length") {
            Some(val.to_str()?.parse::<u64>())
        } else {
            None
        };

        let mut cnt = 0;
        loop {
            let mut buffer = vec![0; self.conf.chunk_size as usize];
            let bcount = resp.read(&mut buffer[..])?;
            cnt += bcount;
            buffer.truncate(bcount);
            if !buffer.is_empty() {
                self.send_content(buffer.as_slice())?;
            } else {
                break;
            }
        }

        for hk in &self.hooks {
            hk.borrow_mut().on_finish();
        }

        Ok(())
    }

    fn send_content(&self, contents: &[u8]) -> Fallible<()> {
        for hk in &self.hooks {
            hk.borrow_mut().on_content(contents);
        }
        Ok(())
    }
}
