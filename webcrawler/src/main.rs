use async_std::task;
use html5ever::tokenizer::{
    BufferQueue, Tag, TagKind, TagToken, Token, TokenSink, TokenSinkResult, Tokenizer,
    TokenizerOpts,
};
use std::borrow::Borrow;
use surf;
use url::{ParseError, Url};

type CrawlResult = Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
type BoxFuture = std::pin::Pin<Box<dyn std::future::Future<Output = CrawlResult> + Send>>;

#[derive(Default, Debug)]
struct LinkQueue {
    links: Vec<String>,
}

impl TokenSink for &mut LinkQueue {
    type Handle = ();
    // <a href="link">test</a>
    fn process_token(&mut self, token: Token, _line_number: u64) -> TokenSinkResult<Self::Handle> {
        match token {
            TagToken(
                tag @ Tag {
                    kind: TagKind::StartTag,
                    ..  
                },
            ) => {
                if tag.name.as_ref() == "a" {
                    for attr in tag.attrs.iter() {
                        if attr.name.local.as_ref() == "href" {
                            let url_str: &[u8] = attr.value.borrow();
                            self.links
                                .push(String::from_utf8_lossy(url_str).into_owned());
                        }
                    }
                }
            }
            _ => {}
        }
        TokenSinkResult::Continue
    }
}

pub fn get_links(url: &Url, page: String) -> Vec<Url> {
    let mut domain_url = url.clone();
    domain_url.set_path("");
    domain_url.set_query(None);

    println!("domain url {:?}", domain_url);

    let mut q = LinkQueue::default();
    let mut tokenizer = Tokenizer::new(&mut q, TokenizerOpts::default());
    let mut buffer = BufferQueue::new();
    buffer.push_back(page.into());
    let _ = tokenizer.feed(&mut buffer);
    q.links
        .iter()
        .map(|link| match Url::parse(link) {
            Err(ParseError::RelativeUrlWithoutBase) => domain_url.join(link).unwrap(),
            Err(_) => panic!("Malformed link found: {}", link),
            Ok(url) => url,
        })
        .collect()
}

fn box_crawl(pages: Vec<Url>,  current: u8, max: u8) -> BoxFuture {
    Box::pin(crawl(pages, current, max))
}

async fn crawl(pages: Vec<Url>, current: u8, max: u8) -> CrawlResult {
    if current > max {
        println!("Reached max depth");
        return Ok(());
    }
    let mut tasks = vec![];
    for url in pages {
        let task = task::spawn(async move { 
            println!("getting {}", url);
            let mut res = surf::get(&url).await?;
            let body = res.body_string().await?;
            let links = get_links(&url, body);
            println!("following: {:?}", links);
            box_crawl(links, current + 1, max).await
        });
        tasks.push(task);
    }

    for task in tasks.into_iter() {
       task.await?;
    }

    Ok(())
}

fn main() {
    let pages = vec![Url::parse("https://www.rust-lang.org").unwrap()];
    task::block_on(async {
        box_crawl(pages, 1, 2).await;
    });
}
