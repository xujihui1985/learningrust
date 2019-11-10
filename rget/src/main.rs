use clap::{
    App,
    Arg,
};
use indicatif::{ProgressBar, ProgressStyle};
use url::{ParseError, Url};

use rget::download::download;

fn main() {
    let matches = App::new("Rget")
        .version("0.1.0")
        .author("Sean")
        .about("wget written in rust")
        .arg(Arg::with_name("URL")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("url to download"))
        .get_matches();

    let url = matches.value_of("URL").unwrap();
    let u = Url::parse(url).unwrap();
//    println!("URL is {}", url);
    download::http_download(u, &matches, "0.1.0");
}

fn create_progress_bar(quite_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = match quite_mode {
        true => ProgressBar::hidden(),
        false => {
            match length {
                Some(len) => ProgressBar::new(len),
                None => ProgressBar::new_spinner(),
            }
        }
    };
    bar.set_message(msg);
    match length.is_some() {
        true => bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}")
                .progress_chars("=> "),
        ),
        false => bar.set_style(ProgressStyle::default_spinner()),
    };
    bar
}

fn parse_url(u: &str) -> Result<url::Url, url::ParseError> {
    match url::Url::parse(u) {
        Ok(url) => Ok(url),
        Err(error) if error == ParseError::RelativeUrlWithoutBase => {
            let url_with_base = format!("{}{}", "http://", u);
            url::Url::parse(url_with_base.as_str())
        }
        Err(error) => Err(error),
    }
}

