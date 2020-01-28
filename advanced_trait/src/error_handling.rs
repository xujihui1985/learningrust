use crate::error_handling::ParseMoneyError::{NoStringErr, TwoPointsErr, NonDigitErr, TooFarErr};

#[derive(Debug, PartialEq)]
pub enum ParseMoneyError {
    SymbolErr,
    NoStringErr,
    TwoPointsErr,
    NonDigitErr(char),
    TooFarErr,
}

pub fn parse_sym_money(s: &str, sym: char, dpoint:usize) -> Result<i32, ParseMoneyError> {
    let (c,v) = parse_money(s, dpoint)?;
    if c != sym {
        return Err(ParseMoneyError::SymbolErr);
    }
    Ok(v)
}

pub fn parse_money(s: &str, dpoint: usize) -> Result<(char, i32), ParseMoneyError> {
    let mut it = s.trim().chars();
    let mut neg = false;

    let mut r_sym = it.next().ok_or(NoStringErr)?;
    if '-' == r_sym {
        neg = true;
        r_sym = it.next().ok_or(NoStringErr)?;
    }

    let mut res:i32 = 0;
    let mut point_pos:Option<usize> = None;
    for c in it {
        if c == '.' {
            if point_pos != None {
                return Err(TwoPointsErr);
            }
            point_pos = Some(0);
            continue;
        }
        if c < '0' || c > '9' {
            return Err(NonDigitErr(c));
        }
        res *= 10;
        res += c as i32 - 48;

        if let Some(pp) = point_pos {
            point_pos = Some(pp + 1);
            if pp >= dpoint {
                return Err(TooFarErr);
            }
        }
    }

    for _ in point_pos.unwrap_or(0)..dpoint {
        res *= 10;
    }

    if neg {
        res = -res;
    }

    Ok((r_sym, res))
}