use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::Context;
use clap::Parser;
use rand::{rngs::StdRng, seq::SliceRandom, RngCore, SeedableRng};
use regex::RegexBuilder;
use walkdir::WalkDir;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(required(true), value_name = "FILE")]
    sources: Vec<String>,

    #[arg(short('m'), long)]
    pattern: Option<String>,

    #[arg(short, long)]
    insensitive: bool,

    #[arg(short, long, value_parser(clap::value_parser!(u64)))]
    seed: Option<u64>,
}

struct Fortune {
    source: String,
    text: String,
}

fn read_fortunes(paths: &[PathBuf]) -> anyhow::Result<Vec<Fortune>> {
    let mut fortunes = vec![];
    let mut buffer = vec![];

    for path in paths {
        let basename = path.file_name().unwrap().to_string_lossy().into_owned();
        let file = File::open(path).context("failed to open {path}")?;

        for line in BufReader::new(file).lines().map_while(|l| l.ok()) {
            if line == "%" {
                if !buffer.is_empty() {
                    fortunes.push(Fortune {
                        source: basename.clone(),
                        text: buffer.join("\n"),
                    });
                    buffer.clear();
                }
            } else {
                buffer.push(line);
            }
        }
    }
    Ok(fortunes)
}

fn pick_fortune(fortunes: &[Fortune], seed: Option<u64>) -> Option<String> {
    let mut rng: Box<dyn RngCore> = match seed {
        Some(val) => Box::new(StdRng::seed_from_u64(val)),
        _ => Box::new(rand::thread_rng()),
    };
    fortunes.choose(&mut rng).map(|f| f.text.to_string())
}

fn find_files(paths: &[String]) -> anyhow::Result<Vec<PathBuf>> {
    let dat = OsStr::new("dat");
    let mut files = vec![];
    for p in paths {
        match fs::metadata(p) {
            Err(e) => anyhow::bail!("Failed to read metadata for {p}: {e}"),
            Ok(_) => files.extend(
                WalkDir::new(p)
                    .into_iter()
                    .filter_map(|f| f.ok())
                    .filter(|f| f.file_type().is_file() && f.path().extension() != Some(dat))
                    .map(|f| f.path().into()),
            ),
        }
    }
    files.sort();
    files.dedup();
    Ok(files)
}

fn main() {
    if let Err(err) = run(Args::parse()) {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> anyhow::Result<()> {
    let pattern = args
        .pattern
        .map(|val| {
            RegexBuilder::new(&val)
                .case_insensitive(args.insensitive)
                .build()
                .map_err(|err| anyhow::anyhow!(r#"Invalid pattern {err}"#))
        })
        .transpose()?;

    let files = find_files(&args.sources).context("failed to find files")?;
    let fortunes = read_fortunes(&files).context("failed to read fortunes")?;
    match pattern {
        Some(pattern) => {
            let mut prev_source = None;
            for fortune in fortunes.iter().filter(|f| pattern.is_match(&f.text)) {
                if prev_source.as_ref().map_or(true, |s| s != &fortune.source) {
                    println!("==> {} <==\n", fortune.source);
                    prev_source = Some(fortune.source.clone());
                }
                println!("{}\n%", fortune.text);
            }
        }
        None => {
            if let Some(fortune) = pick_fortune(&fortunes, args.seed) {
                println!("{fortune}");
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_files() {
        let files = find_files(&["tests/input/jokes".into()]).expect("failed to find files");
        assert_eq!(files.len(), 2);
    }
}
