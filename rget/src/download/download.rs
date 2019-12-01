use std::collections::HashMap;

use crate::download::bar::create_progress_bar;
use crate::download::core;
use crate::download::core::HttpDownload;
use clap::ArgMatches;
use console::style;
use failure::{format_err, Fallible};
use indicatif::{HumanBytes, ProgressBar};
use reqwest::header::HeaderMap;
use reqwest::ClientBuilder;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::time::Duration;
use url::Url;

type Headers = HashMap<String, String>;

struct DefaultEventHandler {
    prog_bar: Option<ProgressBar>,
    fname: String,
    file: BufWriter<fs::File>,
}

fn get_file_handle(fname: &str) -> std::io::Result<fs::File> {
    OpenOptions::new().write(true).create(true).open(fname)
}

impl DefaultEventHandler {
    pub fn new(fname: &str) -> Fallible<DefaultEventHandler> {
        Ok(DefaultEventHandler {
            prog_bar: None,
            fname: fname.to_string(),
            file: BufWriter::new(get_file_handle(fname)?),
        })
    }

    fn create_prog_bar(&mut self, length: Option<u64>) {
        if let Some(len) = length {
            let exact = style(len).green();
            let human_readable = style(format!("{}", HumanBytes(len))).red();
            println!("Length: {} ({})", exact, human_readable);
        } else {
            println!("Length: {}", style("unknown").red());
        }

        self.prog_bar = Some(create_progress_bar(&self.fname, length));
    }
}

impl core::EventsHandler for DefaultEventHandler {
    fn on_headers(&mut self, header: &HeaderMap) {
        match header.get("Content-Type") {
            Some(ct) => println!("Type: {}", style(ct.to_str().unwrap()).green()),
            None => println!("Type: unknown"),
        }
        match header.get("Content-Length") {
            Some(cl) => self.create_prog_bar(cl.to_str().unwrap().parse::<u64>().ok()),
            None => println!("{}", style("content-length is empty").red()),
        }
    }

    fn on_content(&mut self, content: &[u8]) -> Fallible<()> {
        let byte_count = content.len() as u64;
        self.file.write_all(content)?;
        if let Some(b) = &self.prog_bar {
            b.inc(byte_count);
        }
        Ok(())
    }
}

pub fn http_download(url: Url, args: &ArgMatches, version: &str) -> Fallible<()> {

    let resume_download = args.is_present("continue");
    let concurrent_download = args.is_present("singlethread");
    let user_agent = args
        .value_of("AGENT")
        .unwrap_or(&format!("RGET/{}", version))
        .to_string();
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

    let headers = prep_headers(&fname, resume_download, &user_agent)?;

    let state_file_exits = Path::new(&format!("{}.st", fname)).exists();
    let chunk_size = 512_000u64;

    let chunk_offsets = if state_file_exits && resume_download && concurrent_download && cl != 0 {
        Some(get_resume_chunk_offsets(&fname, cl, chunk_size)?)
    } else {
        None
    };

    let bytes_on_disk = if resume_download {
        calc_bytes_on_disk(&fname)?
    } else {
        None
    };

    let conf = core::Config {
        user_agent,
        resume: resume_download,
        headers,
        fname: fname.clone(),
        timeout,
        concurrent: concurrent_download,
        max_retries: 100,
        num_workers,
        bytes_on_disk,
        chunk_offsets,
        chunk_size,
    };

    let mut client = HttpDownload::new(url, conf)?;
    let h = DefaultEventHandler::new(&fname)?;
    client.events_hook(h).download()?;
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
    let cd = headers
        .get("Content-Disposition")
        .and_then(|val| {
            if val.contains("filename=") {
                Some(val)
            } else {
                None
            }
        })
        .and_then(|val| {
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
                    name.to_string()
                }
            }
        },
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

fn get_resume_chunk_offsets(fname: &str, cl: u64, chunk_size: u64) -> Fallible<Vec<(u64, u64)>> {
    let st_name = format!("{}.st", fname);
    let input = fs::File::open(st_name)?;
    let buf = BufReader::new(input);
    let mut downloaded = vec![];
    for line in buf.lines() {
        //        let l = line?.split(":").collect::<Vec<_>>();
        let l = line?;
        let l = l.split(":").collect::<Vec<_>>();
        let n = (l[0].parse::<u64>()?, l[1].parse::<u64>()?);
        downloaded.push(n);
    }
    downloaded.sort_by_key(|a| a.1);
    let mut chunks = vec![];
    let mut i = 0u64;
    for (bc, offset) in downloaded {
        if i == offset {
            i = offset + bc;
        } else {
            chunks.push((i, offset - 1));
            i = offset + bc;
        }
    }

    while (cl - i) > chunk_size {
        chunks.push((i, i + chunk_size - 1));
        i += chunk_size;
    }
    chunks.push((i, cl));
    Ok(chunks)
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
            _ => Ok(None),
        }
    }
}
