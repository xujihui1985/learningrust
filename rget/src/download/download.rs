use std::collections::HashMap;

use clap::ArgMatches;
use failure::_core::time::Duration;
use failure::{Fallible, format_err};
use reqwest::ClientBuilder;
use url::Url;
use std::path::Path;
use std::fs;
use std::io::{BufReader, BufRead};

type Headers = HashMap<String, String>;

pub fn http_download(url: Url, args: &ArgMatches, version: &str) -> Fallible<()> {
    let resume_download = args.is_present("continue");
    let concurrent_download = args.is_present("singlethread");
    let user_agent = args.value_of("AGENT").unwrap_or(&format!("RGET/{}", version)).to_string();
    let timeout_str = args.value_of("SECONDS").unwrap_or("30");
    let timeout = timeout_str.parse::<u64>()?;
    let number_of_workers_str = args.value_of("NUM_CONNECTION").unwrap_or("8");
    let num_workers = number_of_workers_str.parse::<usize>()?;

    let headers = request_headers_from_server(&url, timeout, &user_agent)?;
    let output = args.value_of("FILE");
    let fname = gen_filename(&url, output, &headers);
    println!("fname is {}", fname);

    if args.is_present("headers") {
        println!("{:?}", headers);
        return Ok(());
    }
    let cl = if let Some(val) = headers.get("Content-Length") {
        val.parse::<u64>().unwrap_or(0)
    } else {
        0u64
    };

    let headers = prep_headers(&fname, resume_download, &user_agent);


    Ok(())
}

fn request_headers_from_server(url: &Url, timeout: u64, use_agent: &str) -> Fallible<Headers> {
    let t = Duration::new(timeout, 0);
    let client = ClientBuilder::new().timeout(t).build()?;
    let resp = client.head(url.as_str()).send()?;
    let mut result = Headers::new();
    for (key, value) in resp.headers().iter() {
        result.insert(key.as_str().to_owned(), String::from(value.to_str()?));
    }
    Ok(result)
}

fn gen_filename(url: &Url, fname: Option<&str>, headers: &Headers) -> String {
    let cd = headers.get("Content-Disposition")
        .and_then(|val| {
            if val.contains("filename=") {
                Some(val)
            } else {
                None
            }
        }).and_then(|val| {
        let y = val.rsplit(';');
        let x = val
            .rsplit(';')
            .nth(0)
            .unwrap_or("")
            .rsplit('=')
            .nth(0)
            .unwrap_or("")
            .trim_start_matches('"')
            .trim_end_matches('"');
        if !x.is_empty() {
            Some(x.to_string())
        } else {
            None
        }
    });
    match fname {
        Some(name) => name.to_string(),
        None => match cd {
            Some(val) => val,
            None => {
                let name = &url.path().split("/").last().unwrap_or("");
                if name.is_empty() {
                    "index.html".to_string()
                } else {
                    "genfrom header".to_string()
                }
            }
        }
    }
}

fn prep_headers(fname: &str, resume: bool, user_agent: &str) -> Fallible<Headers> {
    let bytes_on_disk = calc_bytes_on_disk(fname)?;
    let mut headers = Headers::new();
    if let Some(bcount) = bytes_on_disk {
        if resume {
            let byte_range = format!("bytes={}-", bcount);
            headers.insert(String::from("Range"), byte_range);
        }
    }
    headers.insert(String::from("User-Agent"), user_agent.to_string());
    Ok(headers)
}

fn calc_bytes_on_disk(fname: &str) -> Fallible<Option<u64>> {
    let st_fname = format!("{}.st", fname);
    if Path::new(&st_fname).exists() {
        let input = fs::File::open(st_fname)?;
        let buf = BufReader::new(input);
        let mut byte_count = 0u64;
        for line in buf.lines() {
            let num_of_bytes = line?
                .split(":")
                .nth(0)
                .ok_or_else(|| format_err!("failed to split state file"))?
                .parse::<u64>()?;
            byte_count += num_of_bytes;
        }
        Ok(Some(byte_count))
    } else {
        match fs::metadata(fname) {
            Ok(meta) => Ok(Some(meta.len())),
            _ => Ok(None)
        }
    }
}
