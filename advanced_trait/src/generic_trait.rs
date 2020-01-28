use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;

use crate::error_handling::{parse_sym_money, ParseMoneyError};

#[derive(Debug, PartialEq)]
pub enum USDError {
    ParseError(ParseMoneyError),
    OtherError,
}

impl From<ParseMoneyError> for USDError {
    fn from(e: ParseMoneyError) -> Self {
        USDError::ParseError(e)
    }
}

#[derive(PartialEq, Debug)]
pub struct USD(i32);

#[derive(PartialEq, Debug)]
pub struct RMB(i32);

#[derive(PartialEq, Debug, Clone)]
pub struct GBP(i32);

impl FromStr for USD {
    type Err = USDError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(USD(parse_sym_money(s, '$', 2)?))
    }
}

impl Display for USD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let r = (self.0 as f32) / 100.0;
        if r < 0.0 {
            return write!(f, "-${}", -r);
        }
        write!(f, "${}", r)
    }
}

pub trait ToUSDv<F> {
    fn to_uv(&self, f: F) -> f32;
}

pub trait FromUSDv<F> {
    fn from_uv(&self, v: f32) -> F;
}

impl Account for Ex {
    fn id(&self) -> i32 {
        self.ac_id
    }
}

pub struct Ex {
    ac_id: i32,
    rmb: f32,
    gbp: f32,
}

#[derive(PartialEq, Debug)]
pub struct Transaction<A> {
    from_id: i32,
    to_id: i32,
    amount: A,
}

impl ToUSDv<GBP> for Ex {
    fn to_uv(&self, g: GBP) -> f32 {
        (g.0 as f32) * self.gbp
    }
}

impl FromUSDv<RMB> for Ex {
    fn from_uv(&self, f: f32) -> RMB {
        RMB((f / self.rmb) as i32)
    }
}

pub trait Account {
    fn id(&self) -> i32;
}

pub trait Exchange<F, T> {
    fn convert(&self, f: F) -> T;
}

// implement Exchange trait automaticlly for all type E
// that implement both ToUSDv and FromUSDv
impl<E, F, T> Exchange<F, T> for E
    where E: ToUSDv<F> + FromUSDv<T>
{
    fn convert(&self, f: F) -> T {
        self.from_uv(self.to_uv(f))
    }
}

pub trait ExchangeAccount<F, T> {
    fn exchange(&self, f_id: i32, t_id: i32, amount: F)
                -> (Transaction<F>, Transaction<T>);
}

impl<E, F, T> ExchangeAccount<F, T> for E
    where E: Exchange<F, T> + Account,
          F: Clone
{
    fn exchange(&self, f_id: i32, t_id: i32, amount: F) -> (Transaction<F>, Transaction<T>) {
        let ft = Transaction { from_id: f_id, to_id: self.id(), amount: amount.clone() };
        let tt = Transaction { from_id: self.id(), to_id: t_id, amount: self.convert(amount) };
        (ft, tt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let g = GBP(10);
        let ex = Ex {
            ac_id: 12,
            gbp: 10.0,
            rmb: 0.8,
        };
        // because ex implement both FromUSDv and ToUSDv
        // it automaticlly implement trait Exchange
        let c = ex.convert(g);
        assert_eq!(RMB(125), c);
    }

    #[test]
    fn it_create_transaction() {
        let g = GBP(10);
        let ex = Ex {
            ac_id: 12,
            gbp: 10.0,
            rmb: 0.8,
        };
        let (ft, tt) = ex.exchange(12, 11, g.clone());
        assert_eq!(ft, Transaction {
            from_id: 12,
            to_id: 12,
            amount: g,
        })
    }

    #[test]
    fn it_should_return_formatted_usd() {
        let u = USD(123);
        let str_u = u.to_string();
        assert_eq!(str_u, "$1.23");
    }

    #[test]
    fn it_should_parse_usd() {
        let g = "$123".parse();
        assert_eq!(g, Ok(USD(12300)));
    }
}
