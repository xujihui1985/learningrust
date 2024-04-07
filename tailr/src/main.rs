use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek, Write},
    sync::OnceLock,
};

use anyhow::Context;
use clap::{Arg, ArgAction, Command};
use regex::Regex;

static NUM_RE: OnceLock<Regex> = OnceLock::new();

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    lines: String,
    bytes: Option<String>,
    quite: bool,
}

fn get_args() -> Args {
    let matches = Command::new("tailr")
        .version("0.1.0")
        .author("sean")
        .about("rust version of tail")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input files")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help("number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .conflicts_with("lines")
                .help("number of bytes"),
        )
        .arg(
            Arg::new("quite")
                .short('q')
                .long("quite")
                .action(ArgAction::SetTrue)
                .help("suppress headers"),
        )
        .get_matches();

    Args {
        files: matches
            .get_many("files")
            .expect("files are required")
            .cloned()
            .collect::<Vec<_>>(),
        lines: matches
            .get_one::<String>("lines")
            .expect("lines is required")
            .clone(),
        bytes: matches.get_one::<String>("bytes").cloned(),
        quite: matches.get_flag("quite"),
    }
}

#[derive(Debug, PartialEq)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

fn parse_num(val: String) -> anyhow::Result<TakeValue> {
    let num_re = NUM_RE.get_or_init(|| Regex::new(r"^([+-])?(\d+)$").unwrap());
    // let num_re = Regex::new(r"^([+-]?(\d+)$)").unwrap();
    match num_re.captures(&val) {
        Some(caps) => {
            let sign = caps.get(1).map_or("-", |m| m.as_str());
            let signed_num = format!("{}{}", sign, caps.get(2).unwrap().as_str());
            if let Ok(num) = signed_num.parse() {
                if sign == "+" && num == 0 {
                    Ok(TakeValue::PlusZero)
                } else {
                    Ok(TakeValue::TakeNum(num))
                }
            } else {
                anyhow::bail!("invalid number")
            }
        }
        None => anyhow::bail!("invalid number"),
    }
}

fn main() -> anyhow::Result<()> {
    let args = get_args();
    run(args)
}

fn run(args: Args) -> anyhow::Result<()> {
    let num_files = args.files.len();
    let lines = parse_num(args.lines).context("failed to parse lines")?;
    let bytes = args
        .bytes
        .map(parse_num)
        .transpose()
        .context("failed to parse bytes")?;

    for (file_no, filename ) in args.files.iter().enumerate() {
        match File::open(&filename) {
            Ok(file) => {
                if !args.quite && num_files > 1 {
                    println!("{}==> {filename} <==", if file_no == 0 { "" } else { "\n" });
                }
                let (total_lines, _total_bytes) = count_lines_bytes(&filename)?;
                let file = BufReader::new(file);
                if let Some(ref bytes) = bytes {
                    print_bytes(file, bytes, _total_bytes).context("failed to print bytes")?;
                } else {
                    print_lines(file, &lines, total_lines as u64).context("failed to print lines")?;
                }
            },
            Err(err) => eprintln!("{filename}: {err}"),
        }
    }

    Ok(())
}

fn count_lines_bytes(filename: &str) -> anyhow::Result<(i64, i64)> {
    let mut file = BufReader::new(File::open(filename)?);
    let mut num_lines = 0;
    let mut num_bytes = 0;
    let mut buf = Vec::new();
    loop {
        let bytes_read = file.read_until(b'\n', &mut buf)?;
        if bytes_read == 0 {
            break;
        }
        num_lines += 1;
        num_bytes += bytes_read as i64;
        buf.clear();
    }
    Ok((num_lines, num_bytes))
}

fn print_lines(
    mut file: impl BufRead,
    num_lines: &TakeValue,
    total_lines: u64,
) -> anyhow::Result<()> {
    if let Some(start) = get_start_index(num_lines, total_lines) {
        let mut line_num = 0;
        let mut buf = Vec::new();
        loop {
            let bytes_read = file.read_until(b'\n', &mut buf)?;
            if bytes_read == 0 {
                break;
            }
            if line_num >= start {
                std::io::stdout().write_all(&buf)?;
            }
            line_num += 1;
            buf.clear();
        }
    }
    Ok(())
}

fn print_bytes(
    mut file: impl Read + Seek,
    num_bytes: &TakeValue,
    total_bytes: i64,
) -> anyhow::Result<()> {
    if let Some(start) = get_start_index(num_bytes, total_bytes as u64) {
        file.seek(std::io::SeekFrom::Start(start))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        if !buf.is_empty() {
            std::io::stdout().write_all(&buf)?;
        }
    }
    Ok(())
}

fn get_start_index(take_val: &TakeValue, total: u64) -> Option<u64> {
    match take_val {
        TakeValue::PlusZero => {
            if total > 0 {
                Some(0)
            } else {
                None
            }
        },
        TakeValue::TakeNum(num) => {
            let num = *num;
            if num == 0 || total == 0 || num > total as i64 {
                None
            } else {
                let start = if num < 0 {
                    total as i64 + num
                } else {
                    num - 1
                };
                Some(if start < 0 { 0 } else {start as u64})
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_parse_num() {
        let res = parse_num("3".into());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeValue::TakeNum(-3));

        let res = parse_num("+3".into());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeValue::TakeNum(3));

        let res = parse_num("-3".into());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeValue::TakeNum(-3));

        let res = parse_num("0".into());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeValue::TakeNum(0));

        let res = parse_num("+0".into());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeValue::PlusZero);
    }

    #[test]
    fn get_count_lines_bytes() {
        let res = count_lines_bytes("tests/input/one.txt").expect("failed to count lines and bytes");
        assert_eq!(res, (2, 11));
    }

    #[test]
    fn test_get_start_index() {
        assert_eq!(get_start_index(&TakeValue::PlusZero, 0), None);
        assert_eq!(get_start_index(&TakeValue::PlusZero, 1), Some(0));
        assert_eq!(get_start_index(&TakeValue::TakeNum(0), 1), None);
        assert_eq!(get_start_index(&TakeValue::TakeNum(1), 0), None);
        assert_eq!(get_start_index(&TakeValue::TakeNum(2), 1), None);
    }

    #[test]
    fn test_ref_equal() {
        let val = 17;
        assert_eq!(&val, &17);
    }
}
